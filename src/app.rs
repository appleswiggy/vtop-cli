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
    pub fn new(id: usize) -> Tab {
        Tab {
            title: format!("Tab {}", id),
            state: TabState::default(),
        }
    }
    pub fn update_title(&mut self, id: usize) {
        self.title = format!("Tab {}", id);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    pub network_event_tx: tokio::sync::mpsc::Sender<NetworkEvent>,
    pub is_loading: bool,
    pub state: AppState,
}

impl App {
    pub fn new(network_event_tx: tokio::sync::mpsc::Sender<NetworkEvent>) -> App {
        let is_loading = false;
        App {
            network_event_tx,
            is_loading,
            state: AppState::default(),
        }
    }

    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Key::Ctrl(ch) = key {
            if ch == 'C' || ch == 'c' {
                return AppReturn::Exit;
            }
            if ch == 'T' || ch == 't' {
                self.new_tab();
            }
            if ch == 'W' || ch == 'w' {
                self.delete_tab();
            }
        }
        if let Key::Left = key {
            if self.state.active_tab == 0 {
                self.state.active_tab = self.state.tabs.len() - 1;
            } else {
                self.state.active_tab = self.state.active_tab - 1;
            }
        }

        if let Key::Right = key {
            self.state.active_tab = (self.state.active_tab + 1) % self.state.tabs.len();
        }

        return AppReturn::Continue;
    }

    pub async fn update_on_tick(&mut self) -> AppReturn {
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
        AppReturn::Continue
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

            self.state.tabs.push(Tab::new(tabs_len + 1));
        }
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

            self.rename_all_tabs();
        }
    }

    pub fn rename_all_tabs(&mut self) {
        for i in 1..(self.state.tabs.len() + 1) {
            self.state.tabs.get_mut(i - 1).unwrap().update_title(i);
        }
    }
}
