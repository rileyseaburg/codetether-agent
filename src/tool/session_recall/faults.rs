//! Fault classification and result helpers for session recall.

use crate::session::Fault;
use crate::tool::ToolResult;
use serde_json::json;

/// Map an error to a typed fault.
pub fn fault_from_error(err: &anyhow::Error) -> Fault {
    let lowered = err.to_string().to_ascii_lowercase();
    if lowered.contains("no session")
        || lowered.contains("not found")
        || lowered.contains("no such file")
    {
        Fault::NoMatch
    } else {
        Fault::BackendError {
            reason: err.to_string(),
        }
    }
}

/// Build a `ToolResult` tagged with fault metadata.
pub fn fault_result(fault: Fault, output: impl Into<String>) -> ToolResult {
    ToolResult::error(output)
        .with_metadata("fault_code", json!(fault.code()))
        .with_metadata("fault_detail", json!(fault.to_string()))
}
