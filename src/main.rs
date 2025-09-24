use std::io;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use futures::{StreamExt, TryFutureExt};
use ratatui::{backend::CrosstermBackend, Terminal};
use serde::ser::StdError;

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
mod commands;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events: &mut EventHandler = &mut EventHandler::new(250);
    let mut tui = Tui::new(terminal);

    /*    let (broadcast_sender, broadcast_receiver) =
            tokio::sync::broadcast::channel(100);*/

    tui.init()?;


    // Start the main loop.
    while app.running {

        // Handle events.
        match events.next().await? {
            Event::UiUpdate(content) => {
                tui.draw_update(&mut app, content);
            }
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                handle_key_events(&key_event, &mut app, Arc::new(events))
                    .unwrap_or_else(|e| { AppResult::from(Result::<Event, Box<dyn StdError>>::Err(e)); });

                // Render the user interface.
                tui.draw(&mut app)?;
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
