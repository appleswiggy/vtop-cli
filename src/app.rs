use crate::{
    input::Key,
    network::NetworkEvent,
    state::{AppState, TabState},
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
        }
    }

    pub async fn handle_global_keys(&mut self, key: Key) {
        if let Key::Ctrl(ch) = key {
            match ch {
                'C' | 'c' => self.exit_app = true,
                'T' | 't' => self.new_tab(),
                'W' | 'w' => self.delete_tab(),
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
            if self.state.active_tab == 0 {
                self.state.active_tab = self.state.tabs.len() - 1;
            } else {
                self.state.active_tab = self.state.active_tab - 1;
            }
            self.key_processed = Some(true);
        }

        if let Key::CtrlRight | Key::CtrlDown = key {
            self.state.active_tab = (self.state.active_tab + 1) % self.state.tabs.len();
            self.key_processed = Some(true);
        }
    }

    pub async fn do_action(&mut self, key: Key) {
        self.key_processed = Some(false);
        self.state.dispatch_notification(key.to_string()); // DEBUG

        self.handle_global_keys(key).await;

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
            self.state.active_tab = tabs_len;

            self.state.tabs.push(Tab::new("New Tab".to_string()));
        } else {
            self.state
                .dispatch_notification("Maximum of 10 tabs are allowed!".to_string());
        }

        self.key_processed = Some(true);
    }

    pub fn delete_tab(&mut self) {
        if self.state.tabs.len() > 1 {
            let new_active_tab = if self.state.active_tab == self.state.tabs.len() - 1 {
                self.state.active_tab - 1
            } else {
                self.state.active_tab
            };

            self.state.tabs.remove(self.state.active_tab);
            self.state.active_tab = new_active_tab;
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
            self.state.active_tab = index;
        }

        self.key_processed = Some(true);
    }
}
