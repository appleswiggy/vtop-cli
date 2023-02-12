use std::sync::Arc;

use crate::app::App;

pub enum NetworkEvent {}

pub struct NetworkHandler<'a> {
    pub app: &'a Arc<tokio::sync::Mutex<App>>,
}

impl<'a> NetworkHandler<'a> {
    pub fn new(app: &'a Arc<tokio::sync::Mutex<App>>) -> NetworkHandler {
        NetworkHandler { app }
    }

    pub async fn handle_network_event(&mut self, _network_event: NetworkEvent) {
        // TODO handle all types of network events

        let mut app = self.app.lock().await;
        app.loaded();
    }
}
