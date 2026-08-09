//! Structured tool failures for rejected pre-approval diagnostics.

use crate::tool::ToolResult;
use serde_json::json;

pub(super) fn diagnostics(tool: &str, issues: Vec<String>) -> ToolResult {
    let rendered = issues
        .iter()
        .take(20)
        .map(|issue| format!("- {issue}"))
        .collect::<Vec<_>>()
        .join("\n");
    let message = format!(
        "Approval was not shown because the proposed code has language-server errors. \
Fix every diagnostic and retry the tool.\n{rendered}"
    );
    ToolResult::structured_error("LSP_PREAPPROVAL_FAILED", tool, &message, None, None)
        .with_metadata("approval_suppressed", json!(true))
        .with_metadata("lsp_diagnostics", json!(issues))
}

pub(super) fn invalid_input(tool: &str, error: &anyhow::Error) -> ToolResult {
    ToolResult::structured_error(
        "PREAPPROVAL_INVALID_INPUT",
        tool,
        &format!("Approval was not shown: {error:#}"),
        None,
        None,
    )
    .with_metadata("approval_suppressed", json!(true))
}
