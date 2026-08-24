//! Structured tool failures for invalid pre-approval input.

use crate::tool::ToolResult;
use serde_json::json;

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
