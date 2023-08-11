use crate::app::{App, AppResult};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_events(event: CrosstermEvent, app: &mut App) -> AppResult<()> {
    match event {
        CrosstermEvent::Key(key_event) => {
            // Component-specified keybindings that only works when certain component
            // is focused.
            app.scene.handle_key(key_event);

            // Global keybindings that works everywhere.
            match key_event.code {
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                // Other handlers you could add here.
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
