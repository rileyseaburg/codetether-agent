//! Async editor keys that require LSP round-trips (go-to-definition, hover).
//!
//! Runs before the synchronous [`handle_editor_key`](super::editor_key) so that
//! `F12`/`gd` (definition), `K` (hover), and hover dismissal are handled with
//! `await` access to the language server. Returns `true` when consumed.

use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Handles LSP-backed editor keys. Returns `true` if the key was consumed.
pub(crate) async fn handle_editor_lsp_key(app: &mut App, cwd: &Path, key: KeyEvent) -> bool {
    if app.state.view_mode != ViewMode::Editor {
        return false;
    }
    // Dismiss an open hover popup with Esc before anything else.
    if app.state.editor_lsp.has_hover() && matches!(key.code, KeyCode::Esc) {
        app.state.editor_lsp.clear_hover();
        app.state.needs_redraw = true;
        return true;
    }
    match (key.code, key.modifiers) {
        (KeyCode::F(12), _) => {
            super::editor_lsp::goto_definition(app, cwd).await;
            true
        }
        (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
            super::editor_lsp_hover::hover(app, cwd).await;
            true
        }
        _ => false,
    }
}
