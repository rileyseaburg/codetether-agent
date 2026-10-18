//! Async editor keys that require LSP round-trips (go-to-definition, hover).
//!
//! Runs before the synchronous editor handling so that
//! `F12`/`gd` (definition), `K` (hover), and hover dismissal are handled with
//! `await` access to the language server. Returns `true` when consumed.

use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::Controls;
use codetether_lsp_core::manager::ManagedClient;

/// Handles LSP-backed editor keys. Returns `true` if the key was consumed.
///
/// # Arguments
/// * `state` — Borrowed frontend editor state and mode guards.
/// * `cwd` — Workspace root used when creating the language-server manager.
/// * `key` — Incoming key event.
/// # Returns
/// Whether the key was consumed.
/// # Examples
/// ```rust,no_run
/// # async fn example<C: codetether_lsp_core::manager::ManagedClient>(
/// # state: &mut codetether_editor_lsp::interactive::Controls<'_, C>,
/// # key: crossterm::event::KeyEvent,
/// # ) {
/// let _ = codetether_editor_lsp::interactive::handle_editor_lsp_key(state, std::path::Path::new("."), key).await;
/// # }
/// ```
pub async fn handle_editor_lsp_key(
    state: &mut Controls<'_, impl ManagedClient>,
    cwd: &Path,
    key: KeyEvent,
) -> bool {
    if !state.in_editor {
        return false;
    }
    if state.approval_editing && matches!(key.code, KeyCode::F(12)) {
        *state.status = "Go-to-definition is disabled while revising an approval proposal".into();
        *state.needs_redraw = true;
        return true;
    }
    // Dismiss an open hover popup with Esc before anything else.
    if state.editor_lsp.has_hover() && matches!(key.code, KeyCode::Esc) {
        state.editor_lsp.clear_hover();
        *state.needs_redraw = true;
        return true;
    }
    match (key.code, key.modifiers) {
        (KeyCode::F(12), _) => {
            super::definition::goto_definition(state, cwd).await;
            true
        }
        (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
            super::hover::hover(state, cwd).await;
            true
        }
        _ => false,
    }
}
