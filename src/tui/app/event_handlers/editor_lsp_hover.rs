//! Async LSP hover (JSDoc/docstring) for the in-TUI editor.
//!
//! Issues a `textDocument/hover` request at the editor cursor and stores the
//! returned markdown in `editor_lsp.hover` for the popup renderer to display.
//! Shares [`ensure_manager`](super::editor_lsp::ensure_manager) and
//! [`cursor_target`](super::editor_lsp::cursor_target) with go-to-definition.

use std::path::Path;

use crate::lsp::LspActionResult;
use crate::tui::app::state::App;

use super::editor_lsp::{cursor_target, ensure_manager};

/// Runs hover at the editor cursor and stores rendered markdown for a popup.
pub(crate) async fn hover(app: &mut App, cwd: &Path) {
    let Some((path, line, col)) = cursor_target(app) else {
        return;
    };
    let manager = ensure_manager(app, cwd);
    let Ok(client) = manager.get_client_for_file(&path).await else {
        app.state.status = "LSP unavailable for hover".to_string();
        return;
    };
    match client.hover(&path, line, col).await {
        Ok(LspActionResult::Hover { contents, .. }) if !contents.trim().is_empty() => {
            app.state.editor_lsp.hover = Some(contents);
        }
        Ok(_) => app.state.status = "No hover info".to_string(),
        Err(e) => app.state.status = format!("Hover failed: {e}"),
    }
    app.state.needs_redraw = true;
}
