use crate::{app::Tab, pages::Page, util::NOTIFICATION_HISTORY_LENGTH};
use std::time::Instant;

pub struct TabState {
    pub blocks: Blocks,
    pub page: Page,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
            blocks: Blocks {
                interactive_blocks: 3,
                active_block: None,
                hovered_block: Some(1),
            },
            page: Page::Debug {
                interactive_blocks: 2,
                active_block: None,
                hovered_block: None,
            },
        }
    }
}

pub enum Session {
    LoggedOut,
    LoggedIn {
        serverid: String,
        jsessionid: String,
        csrf_token: String,
    },
}

pub struct Blocks {
    pub interactive_blocks: usize,
    pub active_block: Option<usize>,
    pub hovered_block: Option<usize>,
}

pub struct Notification {
    pub text: String,
    pub origin_time: Instant,
}

impl Notification {
    pub fn new(text: String) -> Notification {
        Notification {
            text,
            origin_time: Instant::now(),
        }
    }
}

pub struct AppState {
    pub active_tab: usize,
    pub tabs: Vec<Tab>,
    pub session: Session,
    pub notification_scroll: usize,
    pub notifications: Vec<Notification>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            active_tab: 0,
            tabs: vec![Tab {
                title: String::from("New Tab"),
                state: TabState::default(),
            }],
            session: Session::LoggedOut,
            notification_scroll: 0,
            notifications: vec![],
        }
    }
}

impl AppState {
    pub fn is_logged_in(&self) -> bool {
        match self.session {
            Session::LoggedOut => false,
            Session::LoggedIn { .. } => true,
        }
    }

    pub fn dispatch_notification(&mut self, text: String) {
        if self.notifications.len() == NOTIFICATION_HISTORY_LENGTH {
            self.notifications.remove(0);
        }
        self.notifications.push(Notification::new(text));
        self.notification_scroll = 0;
    }
}
