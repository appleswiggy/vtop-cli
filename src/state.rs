use crate::{
    app::Tab,
    pages::{Page, PageBlock},
    util::NOTIFICATION_HISTORY_LENGTH,
};
use std::time::Instant;

pub struct TabState {
    pub page_block: PageBlock,
    pub sidebar_hover: usize,
    pub active_window: Option<Window>,
    pub hovered_window: Option<Window>,
}

impl Default for TabState {
    fn default() -> Self {
        let default_page = Page::Debug;
        let sidebar_hover = Page::iterator()
            .position(|page| page.to_string() == default_page.to_string())
            .unwrap();

        TabState {
            page_block: PageBlock::new(default_page),
            sidebar_hover,
            active_window: None,
            hovered_window: Some(Window::SidebarWindow),
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

#[derive(Clone)]
pub enum Window {
    SidebarWindow,
    PageWindow,
}

pub struct AppState {
    pub selected_tab: usize,
    pub tabs: Vec<Tab>,
    pub session: Session,
    pub notification_scroll: usize,
    pub notifications: Vec<Notification>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            selected_tab: 0,
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
