//! Editor-mode key handling for the TUI event loop.
//!
//! When [`ViewMode::Editor`](crate::tui::models::ViewMode::Editor) is active,
//! key presses are routed to the editor engine
//! ([`map_key`](crate::tui::ui::editor::map_key) +
//! [`apply`](crate::tui::ui::editor::apply::apply)) instead of normal chat
//! handling. On quit the editor buffer is dropped and the view returns to chat.

use std::path::Path;

use crossterm::event::KeyEvent;

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;
use crate::tui::ui::editor::{EditorInput, apply::apply, map_key};

/// Handles a key while in editor mode. Returns `true` if it was consumed.
pub(crate) fn handle_editor_key(app: &mut App, cwd: &Path, key: KeyEvent) -> bool {
    if app.state.view_mode != ViewMode::Editor {
        return false;
    }
    let Some(action) = map_key(key) else {
        return true;
    };
    if action == EditorInput::OpenFinder {
        open_finder_from_editor(app, cwd);
        return true;
    }
    let Some(buf) = app.state.editor.as_mut() else {
        app.state.view_mode = ViewMode::Chat;
        return true;
    };
    match apply(buf, action) {
        Ok(true) => {}
        Ok(false) => {
            app.state.editor = None;
            app.state.editor_scroll = 0;
            app.state.editor_hscroll = 0;
            app.state.view_mode = ViewMode::Chat;
        }
        Err(e) => app.state.status = format!("editor save failed: {e}"),
    }
    app.state.needs_redraw = true;
    true
}

/// Opens the fuzzy file finder without leaving the editor, warning if the
/// current buffer has unsaved changes (the edit is preserved until switch).
fn open_finder_from_editor(app: &mut App, cwd: &Path) {
    let dirty = app.state.editor.as_ref().is_some_and(|b| b.is_dirty());
    crate::tui::app::fuzzy_find::open_fuzzy_find(app, cwd);
    if dirty {
        app.state.status =
            "Unsaved changes — Ctrl+S to save first, or pick a file to switch".to_string();
    }
    app.state.needs_redraw = true;
}
