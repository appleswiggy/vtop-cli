use std::panic;
use std::sync::Arc;

use vtop_cli::app::App;
use vtop_cli::network::NetworkEvent;
use vtop_cli::network::NetworkHandler;
use vtop_cli::panic_hook;
use vtop_cli::start_ui;

#[tokio::main]
async fn main() {
    panic::set_hook(Box::new(|info| {
        panic_hook(info, false);
    }));

    let (network_event_tx, mut network_event_rx) = tokio::sync::mpsc::channel::<NetworkEvent>(100);

    let app = Arc::new(tokio::sync::Mutex::new(App::new(network_event_tx.clone())));
    let app_ui = Arc::clone(&app);

    tokio::spawn(async move {
        let mut network_handler = NetworkHandler::new(&app);
        while let Some(network_event) = network_event_rx.recv().await {
            network_handler.handle_network_event(network_event).await;
        }
    });

    start_ui(&app_ui).await.expect("Failed to start UI.");
}
