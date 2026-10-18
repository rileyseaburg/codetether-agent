//! Keyboard dispatch when the spawn form modal is open.

use std::path::Path;

use codetether_spawn_form::FormAction;
use crossterm::event::KeyEvent;

use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::spawn_form::{close_spawn_form, submit_spawn_form};
use crate::tui::app::state::App;

/// Handle a key press while the spawn form is active.
///
/// Returns `true` if the key was consumed (caller should stop
/// processing). Tab moves fields, Enter submits, Esc cancels,
/// Backspace deletes, and all other chars are inserted.
pub(crate) async fn handle_spawn_form_key(
    app: &mut App,
    cwd: &Path,
    slot: &SessionSlot,
    key: KeyEvent,
) -> bool {
    let Some(form) = app.state.spawn_form.as_mut() else {
        return false;
    };
    match form.handle_key(key) {
        FormAction::Cancel => {
            close_spawn_form(app);
            true
        }
        FormAction::Submit => {
            let form = app.state.spawn_form.take().unwrap();
            let Some(session) = slot.borrow() else {
                app.state.status = "Session is busy; spawn was not run".to_string();
                app.state.spawn_form = Some(form);
                return true;
            };
            submit_spawn_form(app, session, cwd, form).await;
            true
        }
        FormAction::Continue => true,
    }
}
