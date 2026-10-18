//! Reconstruct proposed edits and obtain the host-owned language-server manager.
use crate::{files, report, scan};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::path::Path;

/// Check a proposed tool edit using the host's configured LSP manager.
///
/// # Arguments
/// `workspace` selects configuration and resolves paths; `tool` and `args`
/// describe the proposed write or patch.
///
/// # Returns
/// A blocking result for malformed input or code errors, plus availability warnings.
///
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_lsp_tool::Host>() {
/// let result = codetether_lsp_preflight::inspect::<H>(
///     std::path::Path::new("."), "read", &serde_json::json!({})
/// ).await;
/// assert!(result.0.is_none());
/// # }
/// ```
pub async fn inspect<H: codetether_lsp_tool::Host>(
    workspace: &Path,
    tool: &str,
    args: &Value,
) -> (Option<ToolResult>, Vec<String>) {
    let files = match files::collect(workspace, tool, args) {
        Ok(files) => files,
        Err(error) => return (Some(report::invalid_input(tool, &error)), Vec::new()),
    };
    if files.is_empty() {
        return (None, Vec::new());
    }
    let config = codetether_config::Config::load_for_workspace(workspace)
        .await
        .unwrap_or_default();
    let manager = codetether_lsp_tool::LspTool::<H>::with_config(
        Some(codetether_lsp_core::path_to_uri(workspace)),
        config.lsp,
    )
    .get_manager()
    .await;
    scan::run(workspace, tool, files, manager).await
}
