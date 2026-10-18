//! Async LSP go-to-definition for the in-TUI editor.
//!
//! Translates the editor's `(line, col)` cursor into a `textDocument/definition`
//! request via the shared [`LspManager`], then navigates the editor to the
//! result. Hover lives in [`super::hover::hover`] (SRP).

use std::path::Path;
use std::sync::Arc;

use super::Controls;
use codetether_lsp_core::manager::LspManager;
use codetether_lsp_core::manager::ManagedClient;

/// Ensures the app has a shared [`LspManager`], creating one rooted at `cwd`.
pub(super) fn ensure_manager<C: ManagedClient>(
    state: &mut Controls<'_, C>,
    cwd: &Path,
) -> Arc<LspManager<C>> {
    if let Some(m) = state.editor_lsp.manager.clone() {
        return m;
    }
    let root = codetether_lsp_core::path_to_uri(cwd);
    let manager = Arc::new(LspManager::new(Some(root)));
    state.editor_lsp.manager = Some(manager.clone());
    manager
}
/// Resolve the current buffer cursor through shared LSP coordinate conversion.
pub(super) fn cursor_target(
    state: &Controls<'_, impl ManagedClient>,
) -> Option<(std::path::PathBuf, u32, u32)> {
    crate::cursor_target(state.editor.as_ref())
}

/// Runs go-to-definition at the editor cursor and navigates to the result.
///
/// Language servers often return empty results until indexing finishes, so the
/// actual polling lives in [`super::resolve::resolve_definition`].
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
/// let _ = codetether_editor_lsp::interactive::goto_definition(state, std::path::Path::new(".")).await;
/// # }
/// ```
pub async fn goto_definition(state: &mut Controls<'_, impl ManagedClient>, cwd: &Path) {
    let Some((path, line, col)) = cursor_target(state) else {
        return;
    };
    let manager = ensure_manager(state, cwd);
    let client = match manager.get_client_for_file(&path).await {
        Ok(c) => c,
        Err(e) => {
            *state.status = format!("LSP unavailable: {e}");
            *state.needs_redraw = true;
            return;
        }
    };
    *state.status = "Looking up definition…".to_string();
    super::resolve::resolve_definition(state, client.as_ref(), &path, line, col).await;
}
