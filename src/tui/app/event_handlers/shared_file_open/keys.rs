//! Ctrl-key dispatch for shared-file actions in the Chat view.

use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use super::actions::{goto_shared_file_definition, show_shared_file_diff};

/// Fall-through Ctrl-key handler for shared-file actions.
///
/// `Ctrl+E` shows the shared file's git diff; `Ctrl+F` opens it in the file
/// viewer. Returns `None` for unhandled keys so the caller's match continues.
pub(crate) fn ctrl_key(
    app: &mut App,
    cwd: &Path,
    key: KeyEvent,
    ctrl: bool,
) -> Option<anyhow::Result<bool>> {
    if !ctrl || app.state.view_mode != ViewMode::Chat {
        return None;
    }
    match key.code {
        KeyCode::Char('e') => show_shared_file_diff(app, cwd),
        KeyCode::Char('f') => goto_shared_file_definition(app, cwd),
        _ => return None,
    }
    Some(Ok(false))
}
