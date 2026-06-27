//! Keyboard dispatch when the spawn form modal is open.

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::spawn_form::{close_spawn_form, submit_spawn_form};
use crate::tui::app::state::App;

/// Handle a key press while the spawn form is active.
///
/// Returns `true` if the key was consumed (caller should stop
/// processing). Tab moves fields, Enter submits, Esc cancels,
/// Backspace deletes, and all other chars are inserted.
pub async fn handle_spawn_form_key(app: &mut App, key: KeyEvent) -> bool {
    let Some(form) = app.state.spawn_form.as_mut() else {
        return false;
    };
    match key.code {
        KeyCode::Esc => {
            close_spawn_form(app);
            true
        }
        KeyCode::Tab | KeyCode::BackTab => {
            form.next_field();
            true
        }
        KeyCode::Enter => {
            let form = app.state.spawn_form.take().unwrap();
            submit_spawn_form(app, form).await;
            true
        }
        KeyCode::Backspace => {
            form.backspace();
            true
        }
        KeyCode::Char(c) => {
            form.insert_char(c);
            true
        }
        _ => true, // swallow all other keys while modal is open
    }
}
