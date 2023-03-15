use crate::{
    app::Tab,
    pages::{Block, BlockType, Page, PageBlock},
    util::NOTIFICATION_HISTORY_LENGTH,
};
use std::time::Instant;

pub struct TabState {
    pub page_block: PageBlock,
}

impl Default for TabState {
    fn default() -> Self {
        let default_page = Page::Debug;

        TabState {
            page_block: PageBlock {
                page: default_page,
                block: Block::default(default_page.to_string(), BlockType::ContainerBlock),
            }
            .fill_inner_blocks(),
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
