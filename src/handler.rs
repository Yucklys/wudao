use crate::{
    action::Action,
    app::{App, GameResult},
    ui::scene::SceneType,
};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_term_events(event: CrosstermEvent, app: &mut App) -> GameResult<()> {
    match event {
        CrosstermEvent::Key(key_event) => {
            // Component-specified keybindings that only works when certain component
            // is focused.
            app.handle_key(key_event);

            // Global keybindings that works everywhere.
            match key_event.code {
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                // Show logs
                KeyCode::F(1) => {
                    app.show_log = !app.show_log;
                }
                // Other handlers you could add here.
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

// Handles the actions generated by the game system.
pub fn handle_actions(action: Action, app: &mut App) -> GameResult<()> {
    match action {
        Action::StartNewGame => app.push_scene(SceneType::upgrade_character()),
    }

    Ok(())
}
