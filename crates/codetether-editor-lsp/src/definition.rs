//! Retry definition lookups while the server is still indexing.
use crate::DefinitionOutcome;
use codetether_lsp_core::{LspActionResult, client::LspClient};
use std::path::Path;
/// Poll for the first definition, retrying empty responses six times.
/// # Arguments
/// * `client` - Initialized host-owned LSP client.
/// * `path` - Current file.
/// * `line` - One-based line.
/// * `col` - One-based column.
/// # Returns
/// The first location, a server/transport error, or a no-definition status.
/// # Examples
/// ```no_run
/// # async fn example(client: &codetether_lsp_core::client::LspClient) {
/// let result = codetether_editor_lsp::resolve_definition(client, std::path::Path::new("main.rs"), 1, 1).await;
/// # }
/// ```
pub async fn resolve_definition(
    client: &LspClient,
    path: &Path,
    line: u32,
    col: u32,
) -> DefinitionOutcome {
    for attempt in 0..6u32 {
        match client.go_to_definition(path, line, col).await {
            Ok(LspActionResult::Definition { locations }) if !locations.is_empty() => {
                return DefinitionOutcome::Location(locations[0].clone());
            }
            Ok(LspActionResult::Error { message }) => {
                return DefinitionOutcome::Status(format!("LSP error: {message}"));
            }
            Err(e) => {
                return DefinitionOutcome::Status(format!("Definition failed: {e}"));
            }
            Ok(_) => {
                tracing::debug!(attempt, line, col, "definition empty; retrying");
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        }
    }
    DefinitionOutcome::Status("No definition found (server may still be indexing)".to_string())
}
