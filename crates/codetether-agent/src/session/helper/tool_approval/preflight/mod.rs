//! Application identity adapter for shared pre-approval diagnostics.
use crate::tool::ToolResult;
use serde_json::Value;
use std::path::Path;

pub(super) async fn inspect(
    workspace: &Path,
    tool: &str,
    args: &Value,
) -> (Option<ToolResult>, Vec<String>) {
    codetether_lsp_preflight::inspect::<crate::tool::lsp::ApplicationHost>(workspace, tool, args)
        .await
}

#[cfg(test)]
pub(super) async fn blocked(workspace: &Path, tool: &str, args: &Value) -> Option<ToolResult> {
    inspect(workspace, tool, args).await.0
}

#[cfg(test)]
mod test_suite;
