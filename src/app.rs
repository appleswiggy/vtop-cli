use crate::{input::Key, network::NetworkEvent};

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    network_event_tx: tokio::sync::mpsc::Sender<NetworkEvent>,
    is_loading: bool,
}

impl App {
    pub fn new(network_event_tx: tokio::sync::mpsc::Sender<NetworkEvent>) -> App {
        let is_loading = false;
        App {
            network_event_tx,
            is_loading,
        }
    }

    pub async fn do_action(&mut self, _key: Key) -> AppReturn {
        AppReturn::Continue
    }

    pub async fn update_on_tick(&mut self) -> AppReturn {
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
}
