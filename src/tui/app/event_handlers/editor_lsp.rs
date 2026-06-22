//! Async LSP go-to-definition for the in-TUI editor.
//!
//! Translates the editor's `(line, col)` cursor into a `textDocument/definition`
//! request via the shared [`LspManager`], then navigates the editor to the
//! result. Hover lives in [`editor_lsp_hover`](super::editor_lsp_hover) (SRP).

use std::path::Path;
use std::sync::Arc;

use crate::lsp::{LspActionResult, LspManager};
use crate::tui::app::state::App;

/// Ensures the app has a shared [`LspManager`], creating one rooted at `cwd`.
pub(super) fn ensure_manager(app: &mut App, cwd: &Path) -> Arc<LspManager> {
    if let Some(m) = app.state.editor_lsp.manager.clone() {
        return m;
    }
    let root = crate::lsp::path_to_uri(cwd);
    let manager = Arc::new(LspManager::new(Some(root)));
    app.state.editor_lsp.manager = Some(manager.clone());
    manager
}

/// Resolves the editor's `(path, 1-based line, 1-based col)` for an LSP query.
pub(super) fn cursor_target(app: &App) -> Option<(std::path::PathBuf, u32, u32)> {
    let buf = app.state.editor.as_ref()?;
    let (line, col) = buf.cursor();
    Some((buf.path().to_path_buf(), line as u32 + 1, col as u32 + 1))
}

/// Runs go-to-definition at the editor cursor and navigates to the result.
pub(crate) async fn goto_definition(app: &mut App, cwd: &Path) {
    let Some((path, line, col)) = cursor_target(app) else {
        return;
    };
    let manager = ensure_manager(app, cwd);
    let client = match manager.get_client_for_file(&path).await {
        Ok(c) => c,
        Err(e) => {
            app.state.status = format!("LSP unavailable: {e}");
            return;
        }
    };
    match client.go_to_definition(&path, line, col).await {
        Ok(LspActionResult::Definition { locations }) if !locations.is_empty() => {
            super::editor_lsp_nav::navigate(app, &locations[0]);
        }
        Ok(_) => app.state.status = "No definition found".to_string(),
        Err(e) => app.state.status = format!("Definition failed: {e}"),
    }
    app.state.needs_redraw = true;
}
