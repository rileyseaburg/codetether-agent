//! Retry loop for go-to-definition while a language server is still indexing.
//!
//! Servers (rust-analyzer, tsserver, gopls, …) often return empty location
//! results until their initial index completes. This polls a few times with a
//! short delay, navigating on the first non-empty result.

use std::path::Path;

use crate::lsp::LspActionResult;
use crate::lsp::client::LspClient;
use crate::tui::app::state::App;

/// Polls `go_to_definition` up to a few times, navigating on the first hit.
pub(super) async fn resolve_definition(
    app: &mut App,
    client: &LspClient,
    path: &Path,
    line: u32,
    col: u32,
) {
    for attempt in 0..6u32 {
        match client.go_to_definition(path, line, col).await {
            Ok(LspActionResult::Definition { locations }) if !locations.is_empty() => {
                super::editor_lsp_nav::navigate(app, &locations[0]);
                app.state.needs_redraw = true;
                return;
            }
            Ok(LspActionResult::Error { message }) => {
                app.state.status = format!("LSP error: {message}");
                app.state.needs_redraw = true;
                return;
            }
            Err(e) => {
                app.state.status = format!("Definition failed: {e}");
                app.state.needs_redraw = true;
                return;
            }
            Ok(_) => {
                tracing::debug!(attempt, line, col, "definition empty; retrying");
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        }
    }
    app.state.status = "No definition found (server may still be indexing)".to_string();
    app.state.needs_redraw = true;
}
