use crate::{pages::Page, app::Tab};

pub struct TabState {
    pub page: Page,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
            page: Page::Home
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

pub struct AppState {
    pub active_tab: usize,
    pub tabs: Vec<Tab>,
    pub session: Session,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            active_tab: 0,
            tabs: vec![Tab {
                title: String::from("Tab 1"),
                state: TabState::default(),
            }],
            session: Session::LoggedOut,
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
}
