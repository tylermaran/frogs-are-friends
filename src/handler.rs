use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        // Handle Keypress
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        // Handle backspace
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Enter => {
            app.input.clear();
        }
        // Apparently we need the below (kinda like a default)
        _ => {}
    }
    Ok(())
}
