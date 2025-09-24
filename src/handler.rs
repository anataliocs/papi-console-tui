use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use futures::{SinkExt, TryFutureExt};
use ratatui::text::Line;
use ratatui::widgets::ScrollDirection;

use StellarCliCmdName::{Env, ReadContractDataWasm, Version};

use crate::app::{App, AppResult, ListStates};
use crate::commands::commands::{CmdResponse, execute, StellarCliCmdName};
use crate::commands::commands::StellarCliCmdName::NetworkToggle;
use crate::event::{Event, EventHandler, UiUpdateContent, UiWidget};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: &KeyEvent, app: &mut App, event_handler: Arc<&EventHandler>) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Tab handlers
        KeyCode::Right => {
            app.next_tab();
        }
        KeyCode::Left => {
            app.previous_tab();
        }
        KeyCode::Up => {
            ListStates::select_prev(app.selected_tab, &mut app.list_states);
        }
        KeyCode::Down => {
            ListStates::select_next(app.selected_tab, &mut app.list_states);
        }

        KeyCode::Tab => {
            app.cmd_output_state.cmd_output_scrollbar.scroll(ScrollDirection::Forward);
        }

        KeyCode::Delete => {
            app.cmd_output_state.cmd_output.lines.clear();
        }

        KeyCode::Enter => {
            let stellar_cli_cmd_name: StellarCliCmdName = NetworkToggle;
            // Run the command with a timeout
            let res: CmdResponse = execute(NetworkToggle);

            app.cmd_output_state.cmd_output.push_line(Line::raw(res.raw_cmd.to_string()));
            app.cmd_output_state.cmd_output.push_line(Line::raw(res.result));

            &event_handler.as_ref()
                          .send(Event::UiUpdate(
                              UiUpdateContent::new(UiWidget::CmdOutput,
                                                   String::from("Result"),
                                                   String::from("Network: Local"))))
                          .unwrap_or_else(|e| { drop(e); });

            match stellar_cli_cmd_name {
                Version => {}
                Env => {}
                ReadContractDataWasm => {}
                NetworkToggle => {
                    &event_handler.as_ref()
                                  .send(Event::UiUpdate(
                                      UiUpdateContent::new(UiWidget::Network,
                                                           String::from("Update"),
                                                           String::from("Network: Local"))))
                                  .unwrap_or_else(|e| { e; });

                    &event_handler.as_ref()
                                  .send(Event::UiUpdate(
                                      UiUpdateContent::new(UiWidget::Network,
                                                           String::from("Result"),
                                                           String::from("Connect to local Stellar Network"))))
                                  .unwrap_or_else(|e| { e; });
                }
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
