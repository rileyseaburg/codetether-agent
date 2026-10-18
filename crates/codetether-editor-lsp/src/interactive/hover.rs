//! Async LSP hover (JSDoc/docstring) for the in-TUI editor.
//!
//! Issues a `textDocument/hover` request at the editor cursor and stores the
//! returned markdown in `editor_lsp.hover` for the popup renderer to display.
//! Shares [`ensure_manager`](super::definition::ensure_manager) and
//! [`cursor_target`](super::definition::cursor_target) with go-to-definition.

use std::path::Path;

use super::Controls;
use crate::HoverOutcome;
use codetether_lsp_core::manager::ManagedClient;

use super::definition::{cursor_target, ensure_manager};

/// Runs hover at the editor cursor and stores rendered markdown for a popup.
///
/// # Arguments
/// * `state` — Borrowed frontend editor state and mode guards.
/// * `cwd` — Workspace root used when creating the language-server manager.
/// # Returns
/// Updates editor state with the language-server result or status.
/// # Examples
/// ```rust,no_run
/// # async fn example<C: codetether_lsp_core::manager::ManagedClient>(
/// # state: &mut codetether_editor_lsp::interactive::Controls<'_, C>,
/// # ) {
/// let _ = codetether_editor_lsp::interactive::hover(state, std::path::Path::new(".")).await;
/// # }
/// ```
pub async fn hover(state: &mut Controls<'_, impl ManagedClient>, cwd: &Path) {
    let Some((path, line, col)) = cursor_target(state) else {
        return;
    };
    let manager = ensure_manager(state, cwd);
    let Ok(client) = manager.get_client_for_file(&path).await else {
        *state.status = "LSP unavailable for hover".to_string();
        return;
    };
    match crate::hover(client.as_ref(), &path, line, col).await {
        HoverOutcome::Content(contents) => state.editor_lsp.hover = Some(contents),
        HoverOutcome::Status(status) => *state.status = status,
    }
    *state.needs_redraw = true;
}
