use app::{App, AppReturn};
use backtrace::Backtrace;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use input::KeyEvent;
use std::{
    io::{self, Write},
    panic::{self, PanicInfo},
    sync::Arc,
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod input;
pub mod network;
pub mod pages;
pub mod state;
pub mod ui;
pub mod util;

pub fn panic_hook(info: &PanicInfo<'_>, in_alternate_screen: bool) {
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };

    let mut stdout = io::stdout();

    if in_alternate_screen {
        disable_raw_mode().unwrap();
        queue!(stdout, LeaveAlternateScreen, DisableMouseCapture).unwrap();
    }

    if cfg!(debug_assertions) {
        let location = info.location().unwrap();
        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        queue!(
            stdout,
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            )),
        )
        .unwrap();
    } else {
        queue!(stdout, Print(format!("Error: {}\n\r", msg))).unwrap();
    }

    stdout.flush().unwrap();
}

pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App>>) -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    panic::set_hook(Box::new(|info| {
        panic_hook(info, true);
    }));

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(200);
    let mut events = input::KeyEvents::new(tick_rate);

    let mut first_render = true;

    loop {
        let mut app = app.lock().await;

        if first_render {
            // DO SOMETHING AT FIRST RENDER
            // Like dispatching a notification
            // app.state.dispatch_notification("Long notification string hello hello hello".to_string());
            first_render = false;
        }

        terminal.draw(|rect| ui::draw(rect, &app))?;

        let app_return = match events.next().await {
            KeyEvent::Input(key) => app.do_action(key).await,
            KeyEvent::Tick => app.update_on_tick().await,
        };

        if app_return == AppReturn::Exit {
            events.close();
            break;
        }
    }

    // Restore the terminal and close the application
    terminal.clear()?;
    terminal.show_cursor()?;

    disable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}
