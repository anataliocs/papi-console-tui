use std::cell::RefCell;
use std::fmt::Display;
use std::os::unix::raw::time_t;
use std::ptr::addr_of_mut;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, pin_mut, SinkExt, StreamExt, TryFutureExt};
use tokio::sync::mpsc;
use tokio::time::error::Elapsed;
use tokio::time::Instant;

use crate::app::AppResult;
use crate::event::Event::Tick;

/// Terminal events.
#[derive(Clone, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),

    UiUpdate(UiUpdateContent),
}

#[derive(Clone, Debug, Default)]
pub enum UiWidget {
    #[default]
    NoUpdate,
    Tabs,
    Network,
    ListSelect,
    CmdOutput,
    Scrollbar,
}
#[derive(Clone, Debug, Default)]
pub struct UiUpdateContent {
    ui_widget: UiWidget,
    ui_key: String,
    ui_update_content: String,
}

impl UiUpdateContent {
    pub fn new(ui_widget: UiWidget, ui_key: String, ui_update_content: String) -> Self {
        Self { ui_widget, ui_key, ui_update_content }
    }
}

impl UiUpdatePayload for UiUpdateContent {
    fn ui_widget(&self) -> &UiWidget {
        &self.ui_widget
    }
    fn ui_key(&self) -> &str {
        &self.ui_key
    }
    fn ui_update_content(&self) -> &str {
        &self.ui_update_content
    }
}

pub trait UiUpdatePayload {
    fn ui_widget(&self) -> &UiWidget;
    fn ui_key(&self) -> &str;
    fn ui_update_content(&self) -> &str;
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
    /// Event handler thread.
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                  _ = _sender.closed() => {
                    break;
                  }
                  _ = tick_delay => {
                    _sender.send(Event::Tick).unwrap();
                  }
                  Some(Ok(evt)) = crossterm_event => {
                    match evt {
                      CrosstermEvent::Key(key) => {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                          _sender.send(Event::Key(key)).unwrap();
                        }
                      },
                      CrosstermEvent::Mouse(mouse) => {
                        _sender.send(Event::Mouse(mouse)).unwrap();
                      },
                      CrosstermEvent::Resize(x, y) => {
                        _sender.send(Event::Resize(x, y)).unwrap();
                      },
                      CrosstermEvent::FocusLost => {
                      },
                      CrosstermEvent::FocusGained => {
                      },
                      CrosstermEvent::Paste(_) => {
                      },
                    }
                  }
                }
                ;
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn next(&mut self) -> AppResult<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "This is an IO error",
            )))
    }

    pub fn send(&self, event: Event) -> AppResult<()> {
        self.sender.send(event).map(|e| { e })
            .unwrap_or_else(|e1| { e1; });

        Ok(())
    }
}
