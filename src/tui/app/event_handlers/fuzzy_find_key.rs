//! Keystroke handling for the `/edit` fuzzy file finder overlay.
//!
//! When `app.state.fuzzy_find` is active this consumes all key presses:
//! typing edits the query, Up/Down move the selection, Enter opens the
//! highlighted file, and Esc closes the finder.

use std::path::Path;

use codetether_fuzzy_finder::FinderAction;
use crossterm::event::KeyEvent;

use crate::tui::app::fuzzy_find::confirm_fuzzy_find;
use crate::tui::app::state::App;

/// Handle a key press while the fuzzy finder is open.
///
/// Returns `true` if the key was consumed by the finder.
pub(super) fn handle_fuzzy_find_key(app: &mut App, cwd: &Path, key: KeyEvent) -> bool {
    let Some(finder) = app.state.fuzzy_find.as_mut() else {
        return false;
    };
    match finder.handle_key(key) {
        FinderAction::Cancel => {
            app.state.fuzzy_find = None;
            app.state.status = "Fuzzy open cancelled".to_string();
        }
        FinderAction::Confirm => confirm_fuzzy_find(app, cwd),
        FinderAction::Continue => {}
    }
    app.state.needs_redraw = true;
    true
}
