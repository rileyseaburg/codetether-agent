//! Audit detail builders for tool execution events.
//!
//! Centralizes the JSON detail shape so the typed [`Fault`] reason
//! codes that tools attach via `ToolResult.metadata["fault_code"]`
//! flow into the audit log — and from there into the TUI audit view.
//! Without this surfacing the audit detail would silently swallow the
//! reason code and reduce post-mortem visibility to the tool's
//! free-form output.
//!
//! [`Fault`]: crate::session::Fault

use crate::tool::ToolResult;
use serde_json::{Value, json};

/// Detail JSON for a successful tool invocation. Always includes
/// `duration_ms` and `output_len`; `fault_code` is included when the
/// tool stamped one (typed-fault tools like `session_recall` and the
/// `context_*` family always do).
pub fn tool_success_detail(duration_ms: u64, result: &ToolResult) -> Value {
    json!({
        "duration_ms": duration_ms,
        "output_len": result.output.len(),
        "fault_code": result.metadata.get("fault_code"),
    })
}

/// Detail JSON for a tool invocation that returned an error.
pub fn tool_failure_detail(duration_ms: u64, error: &str) -> Value {
    json!({ "duration_ms": duration_ms, "error": error })
}
