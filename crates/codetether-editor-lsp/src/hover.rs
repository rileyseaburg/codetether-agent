//! Resolve hover content or its user-facing status.
use crate::HoverOutcome;
use codetether_lsp_core::{LspActionResult, client::LspClient};
use std::path::Path;
/// Request hover information and classify content or status.
/// # Arguments
/// * `client` - Initialized host-owned client.
/// * `path` - Current file.
/// * `line` - One-based line.
/// * `col` - One-based column.
/// # Returns
/// Nonempty markdown, or an empty/error status.
/// # Examples
/// ```no_run
/// # async fn example(client: &codetether_lsp_core::client::LspClient) {
/// let result = codetether_editor_lsp::hover(client, std::path::Path::new("main.rs"), 1, 1).await;
/// # }
/// ```
pub async fn hover(client: &LspClient, path: &Path, line: u32, col: u32) -> HoverOutcome {
    match client.hover(path, line, col).await {
        Ok(LspActionResult::Hover { contents, .. }) if !contents.trim().is_empty() => {
            return HoverOutcome::Content(contents);
        }
        Ok(_) => return HoverOutcome::Status("No hover info".to_string()),
        Err(e) => return HoverOutcome::Status(format!("Hover failed: {e}")),
    }
}
