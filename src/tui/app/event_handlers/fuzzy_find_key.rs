//! Keystroke handling for the `/edit` fuzzy file finder overlay.
//!
//! When `app.state.fuzzy_find` is active this consumes all key presses:
//! typing edits the query, Up/Down move the selection, Enter opens the
//! highlighted file, and Esc closes the finder.

use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::fuzzy_find::confirm_fuzzy_find;
use crate::tui::app::state::App;

/// Handle a key press while the fuzzy finder is open.
///
/// Returns `true` if the key was consumed by the finder.
pub(super) fn handle_fuzzy_find_key(app: &mut App, cwd: &Path, key: KeyEvent) -> bool {
    if app.state.fuzzy_find.is_none() {
        return false;
    }
    match key.code {
        KeyCode::Esc => {
            app.state.fuzzy_find = None;
            app.state.status = "Fuzzy open cancelled".to_string();
        }
        KeyCode::Enter => confirm_fuzzy_find(app, cwd),
        KeyCode::Up => move_selection(app, -1),
        KeyCode::Down => move_selection(app, 1),
        KeyCode::Backspace => {
            if let Some(f) = app.state.fuzzy_find.as_mut() {
                f.query.pop();
                f.rerank();
            }
        }
        KeyCode::Char(c) => {
            if let Some(f) = app.state.fuzzy_find.as_mut() {
                f.query.push(c);
                f.rerank();
            }
        }
        _ => {}
    }
    app.state.needs_redraw = true;
    true
}

fn move_selection(app: &mut App, delta: isize) {
    if let Some(f) = app.state.fuzzy_find.as_mut() {
        let len = f.results.len();
        if len == 0 {
            return;
        }
        let next = (f.selected as isize + delta).rem_euclid(len as isize);
        f.selected = next as usize;
    }
}
