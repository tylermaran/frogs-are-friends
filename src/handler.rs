use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_input_command(app: &mut App) {
    match app.input.trim() {
        "upgrade" => {
            app.metal_production += 1;
            println!("Upgraded! Current level: {}", app.metal_production);
        }
        // You can handle other commands here...
        _ => {
            println!("Unknown command: {}", app.input);
        }
    }
}

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
            handle_input_command(app);
            app.input.clear();
        }
        // Apparently we need the below (kinda like a default)
        _ => {}
    }
    Ok(())
}
