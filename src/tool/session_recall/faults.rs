//! Fault classification and result helpers for session recall.

use crate::session::Fault;
use crate::tool::ToolResult;
use serde_json::json;

/// Build a `ToolResult` tagged with fault metadata.
pub fn fault_result(fault: Fault, output: impl Into<String>) -> ToolResult {
    ToolResult::error(output)
        .with_metadata("fault_code", json!(fault.code()))
        .with_metadata("fault_detail", json!(fault.to_string()))
}
