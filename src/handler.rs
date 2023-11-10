use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        // KeyCode::Esc => {
        //     app.quit();
        // }
        // // Exit application on `ESC` or `q`
        // KeyCode::Esc | KeyCode::Char('q') => {
        //     app.quit();
        // }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => {
            // app.increment_counter();
            app.clear_area();
        }
        KeyCode::Left => {
            // app.decrement_counter();
        }
        KeyCode::Char(to_insert) => {
            app.enter_char(to_insert);
        }
        KeyCode::Backspace => {
            app.remove_char();
        }
        KeyCode::Enter => {
            app.next_frame();
        }
        KeyCode::Tab => {
            app.next_frame();
        }
        KeyCode::Esc => {
            app.quit();
        }
        
        
        _ => {}
    }
    Ok(())
}
