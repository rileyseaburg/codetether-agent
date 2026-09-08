//! Language-server gate that runs before an approval reaches the user.

mod background;
mod budget;
mod check;
mod cooldown;
mod diagnostics;
mod files;
mod report;
mod scan;

use std::path::Path;

use crate::tool::{ToolResult, lsp::LspTool};
use serde_json::Value;

#[cfg(test)]
pub(super) async fn blocked(workspace: &Path, tool: &str, args: &Value) -> Option<ToolResult> {
    inspect(workspace, tool, args).await.0
}

pub(super) async fn inspect(
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
    let config = crate::config::Config::load_for_workspace(workspace)
        .await
        .unwrap_or_default();
    let manager = LspTool::with_config(Some(crate::lsp::path_to_uri(workspace)), config.lsp)
        .get_manager()
        .await;
    scan::run(workspace, tool, files, manager).await
}

#[cfg(test)]
mod test_suite;
