//! Language-server gate that runs before an approval reaches the user.

mod diagnostics;
mod files;
mod report;

use std::path::Path;

use crate::lsp::detect_language_from_path;
use crate::tool::{ToolResult, lsp::LspTool};
use serde_json::Value;

pub(super) async fn blocked(workspace: &Path, tool: &str, args: &Value) -> Option<ToolResult> {
    let files = match files::collect(workspace, tool, args) {
        Ok(files) => files,
        Err(error) => return Some(report::invalid_input(tool, &error)),
    };
    if files.is_empty() {
        return None;
    }
    let config = crate::config::Config::load_for_workspace(workspace)
        .await
        .unwrap_or_default();
    let manager = LspTool::with_config(Some(crate::lsp::path_to_uri(workspace)), config.lsp)
        .get_manager()
        .await;
    let mut issues = Vec::new();
    for (path, content) in files {
        if detect_language_from_path(path.to_string_lossy().as_ref()).is_none() {
            continue;
        }
        let client = match manager.get_client_for_file(&path).await {
            Ok(client) => client,
            Err(error) => {
                tracing::warn!(path = %path.display(), %error, "Pre-approval LSP unavailable");
                continue;
            }
        };
        match client.diagnostics_for_content(&path, &content).await {
            Ok(result) => issues.extend(diagnostics::errors(workspace, &path, result)),
            Err(error) => tracing::warn!(path = %path.display(), %error, "Pre-approval LSP failed"),
        }
    }
    (!issues.is_empty()).then(|| report::diagnostics(tool, issues))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
