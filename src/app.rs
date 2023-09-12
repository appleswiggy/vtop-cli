use std::io::stdout;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};

use crate::{
    input::Key,
    network::NetworkEvent,
    pages::{Page, PageBlock},
    state::{AppState, TabState, Window},
    util::{MAXIMUM_TABS, NOTIFICATION_SEPERATOR},
};

pub struct Tab {
    pub title: String,
    pub state: TabState,
}

impl Tab {
    pub fn new(title: String) -> Tab {
        Tab {
            title,
            state: TabState::default(),
        }
    }
    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }
}

pub struct App {
    pub network_event_tx: tokio::sync::mpsc::Sender<NetworkEvent>,
    pub is_loading: bool,
    pub state: AppState,
    pub key_processed: Option<bool>,
    pub exit_app: bool,
    pub mouse_capture: bool,
}

impl App {
    pub fn new(network_event_tx: tokio::sync::mpsc::Sender<NetworkEvent>) -> App {
        let is_loading = false;
        App {
            network_event_tx,
            is_loading,
            state: AppState::default(),
            key_processed: None,
            exit_app: false,
            mouse_capture: true,
        }
    }

    pub async fn handle_global_keys(&mut self, key: Key) {
        if let Key::Ctrl(ch) = key {
            match ch {
                'C' | 'c' => self.exit_app = true,
                'T' | 't' => self.new_tab(),
                'W' | 'w' => self.delete_tab(),
                'E' | 'e' => self.toggle_mouse_capture(),
                _ => (),
            }
        }
        if let Key::Char(ch) = key {
            match ch {
                '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' => self.switch_tab(ch),
                _ => (),
            }
        }
        if let Key::CtrlLeft | Key::CtrlUp = key {
            if self.state.selected_tab == 0 {
                self.state.selected_tab = self.state.tabs.len() - 1;
            } else {
                self.state.selected_tab = self.state.selected_tab - 1;
            }
            self.key_processed = Some(true);
        }

        if let Key::CtrlRight | Key::CtrlDown = key {
            self.state.selected_tab = (self.state.selected_tab + 1) % self.state.tabs.len();
            self.key_processed = Some(true);
        }
    }

    pub fn handle_window_hover(&mut self, key: Key) {
        let tab_state = &mut self.state.tabs[self.state.selected_tab].state;

        if let Key::Tab | Key::Right | Key::Down | Key::ShiftTab | Key::Left | Key::Up = key {
            if let Some(Window::PageWindow) = tab_state.hovered_window {
                tab_state.hovered_window = Some(Window::SidebarWindow)
            } else if let Some(Window::SidebarWindow) = tab_state.hovered_window {
                tab_state.hovered_window = Some(Window::PageWindow);
            }
            self.key_processed = Some(true);
        } else if let Key::Enter = key {
            tab_state.active_window = tab_state.hovered_window.clone();

            // if PageWindow is selected, it's first block should be hovered 
            if let Some(Window::PageWindow) = tab_state.active_window {
                self.state.tabs[self.state.selected_tab]
                    .state
                    .page_block
                    .block
                    .hover_first_block();
            }
            self.key_processed = Some(true);
        }
    }

    pub fn handle_sidebar_input(&mut self, key: Key) {
        let current_sidebar_hover = self.state.tabs[self.state.selected_tab].state.sidebar_hover;

        if let Key::Tab | Key::Right | Key::Down = key {
            self.state.tabs[self.state.selected_tab].state.sidebar_hover =
                (current_sidebar_hover + 1) % Page::iterator().count();
        } else if let Key::ShiftTab | Key::Left | Key::Up = key {
            self.state.tabs[self.state.selected_tab].state.sidebar_hover =
                if current_sidebar_hover == 0 {
                    Page::iterator().count() - 1
                } else {
                    current_sidebar_hover - 1
                };
        } else if let Key::Enter = key {
            // if sidebar_hover doesn't hover the currently opened page, the Enter key
            // opens the new page hovered by sidebar_hover.

            if self.state.tabs[self.state.selected_tab]
                .state
                .page_block
                .page
                .to_string()
                != Page::iterator()
                    .nth(self.state.tabs[self.state.selected_tab].state.sidebar_hover)
                    .unwrap()
                    .to_string()
            {
                let new_page = Page::iterator()
                    .nth(self.state.tabs[self.state.selected_tab].state.sidebar_hover)
                    .unwrap();

                self.state.tabs[self.state.selected_tab].state.page_block =
                    PageBlock::new(new_page.clone());
            }
        } else if let Key::Esc = key {
            // On Esc key, sidebar is no longer selected and sidebar_hover returns to the position
            // of currently opened page.

            self.state.tabs[self.state.selected_tab].state.active_window = None;

            self.state.tabs[self.state.selected_tab].state.sidebar_hover = Page::iterator()
                .position(|page| {
                    page.to_string()
                        == self.state.tabs[self.state.selected_tab]
                            .state
                            .page_block
                            .page
                            .to_string()
                })
                .unwrap();
        }
    }

    pub async fn do_action(&mut self, key: Key) {
        self.key_processed = Some(false);
        self.state.dispatch_notification(key.to_string()); // DEBUG

        self.handle_global_keys(key).await;

        let tab_state = &mut self.state.tabs[self.state.selected_tab].state;

        if tab_state.active_window.is_none() {
            self.handle_window_hover(key);
        } else {
            match tab_state.active_window.as_ref().unwrap() {
                Window::SidebarWindow => self.handle_sidebar_input(key),
                Window::PageWindow => {
                    // if none of the blocks inside PageWindow is selected, Esc causes the
                    // de-selection of PageWindow, else Esc key is transfered to the page
                    // for handling.

                    let mut flag = false;

                    if let Key::Esc = key {
                        if tab_state.page_block.block.has_selected_child() == false {
                            tab_state.page_block.block.deselect();
                            tab_state.active_window = None;
                            flag = true;
                        }
                    }

                    if flag == false {
                        tab_state.page_block.block.handle_input(key);
                    }
                }
            }
        }

        self.key_processed = None;
    }

    pub async fn update_on_tick(&mut self) {
        if self.state.notifications.len() > 0 {
            let notification_length = self.state.notifications[self.state.notifications.len() - 1]
                .text
                .len();

            if self.state.notification_scroll == notification_length + NOTIFICATION_SEPERATOR.len()
            {
                self.state.notification_scroll = 0;
            } else {
                self.state.notification_scroll += 1;
            }
        }
    }

    pub async fn dispatch(&mut self, action: NetworkEvent) {
        self.is_loading = true;
        if let Err(e) = self.network_event_tx.send(action).await {
            self.is_loading = false;
            println!("Error from dispatch {}", e);
        }
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    pub fn new_tab(&mut self) {
        if self.state.tabs.len() < MAXIMUM_TABS {
            let tabs_len = self.state.tabs.len();
            self.state.selected_tab = tabs_len;

            self.state.tabs.push(Tab::new("New Tab".to_string()));
        } else {
            self.state
                .dispatch_notification("Maximum of 10 tabs are allowed!".to_string());
        }

        self.key_processed = Some(true);
    }

    pub fn delete_tab(&mut self) {
        if self.state.tabs.len() > 1 {
            let new_selected_tab = if self.state.selected_tab == self.state.tabs.len() - 1 {
                self.state.selected_tab - 1
            } else {
                self.state.selected_tab
            };

            self.state.tabs.remove(self.state.selected_tab);
            self.state.selected_tab = new_selected_tab;
        }

        self.key_processed = Some(true);
    }

    pub fn switch_tab(&mut self, ch: char) {
        let index: usize = match ch {
            '!' => 0,
            '@' => 1,
            '#' => 2,
            '$' => 3,
            '%' => 4,
            '^' => 5,
            '&' => 6,
            '*' => 7,
            '(' => 8,
            ')' => 9,
            _ => 0,
        };

        if index < self.state.tabs.len() {
            self.state.selected_tab = index;
        }

        self.key_processed = Some(true);
    }

    fn toggle_mouse_capture(&mut self) {
        if self.mouse_capture {
            execute!(stdout(), DisableMouseCapture).expect("Unable to disable mouse capture.");
            self.state
                .dispatch_notification("Mouse capture disabled.".to_string());
            self.mouse_capture = false;
        } else {
            execute!(stdout(), EnableMouseCapture).expect("Unable to enable mouse capture.");
            self.state
                .dispatch_notification("Mouse capture enabled.".to_string());
            self.mouse_capture = true;
        }
    }
}
