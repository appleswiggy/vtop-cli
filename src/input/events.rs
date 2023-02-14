use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use super::key::Key;

pub enum KeyEvent {
    Input(Key),
    Tick,
}

pub struct KeyEvents {
    rx: tokio::sync::mpsc::Receiver<KeyEvent>,
    _tx: tokio::sync::mpsc::Sender<KeyEvent>,
    stop_capture: Arc<AtomicBool>,
}

impl KeyEvents {
    pub fn new(tick_rate: Duration) -> KeyEvents {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let stop_capture = Arc::new(AtomicBool::new(false));

        let event_tx = tx.clone();
        let event_stop_capture = stop_capture.clone();

        tokio::spawn(async move {
            loop {
                if crossterm::event::poll(tick_rate).expect("Failed to read terminal.") {
                    if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                        let key = Key::from(key);
                        if let Err(_) = event_tx.send(KeyEvent::Input(key)).await {
                            panic!("Failed to read terminal");
                        }
                    }
                }

                if event_stop_capture.load(Ordering::Relaxed) {
                    break;
                } 

                if let Err(_) = event_tx.send(KeyEvent::Tick).await {
                    panic!("Failed to read terminal");
                }
            }
        });

        KeyEvents {
            rx,
            _tx: tx,
            stop_capture,
        }
    }

    /// Attempts to read an event.
    pub async fn next(&mut self) -> KeyEvent {
        self.rx.recv().await.unwrap_or(KeyEvent::Tick)
    }

    /// Close
    pub fn close(&mut self) {
        self.stop_capture.store(true, Ordering::Relaxed)
    }
}
