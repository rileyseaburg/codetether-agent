//! Stable audit fields with sorted metadata keys and bounded previews.
use super::digest::{preview, sha256_hex};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};

/// Detail JSON for a successful tool invocation. Always includes
/// `duration_ms` and `output_len`; `fault_code` is included when the
/// tool stamped one (typed-fault tools like `session_recall` and the
/// `context_*` family always do).
/// # Arguments
/// Receives elapsed milliseconds and the tool result or error text.
/// # Returns
/// Stable audit JSON with full-text length/hash and a bounded preview.
/// # Examples
/// ```
/// let result = codetether_tool_core::ToolResult::success("ok");
/// assert_eq!(codetether_tool_reporting::tool_success_detail(1, &result)["output_len"], 2);
/// ```
pub fn tool_success_detail(duration_ms: u64, result: &ToolResult) -> Value {
    let mut metadata_keys = result.metadata.keys().cloned().collect::<Vec<_>>();
    metadata_keys.sort();
    json!({
        "duration_ms": duration_ms,
        "output_len": result.output.len(),
        "output_sha256": sha256_hex(&result.output),
        "output_preview": preview(&result.output),
        "metadata_keys": metadata_keys,
        "fault_code": result.metadata.get("fault_code"),
        "truncated": result.metadata.get("truncated"),
    })
}

/// Detail JSON for a tool invocation that returned an error.
/// # Arguments
/// Receives elapsed milliseconds and the tool result or error text.
/// # Returns
/// Stable audit JSON with full-text length/hash and a bounded preview.
/// # Examples
/// ```
/// assert_eq!(codetether_tool_reporting::tool_failure_detail(1, "failed")["error_len"], 6);
/// ```
pub fn tool_failure_detail(duration_ms: u64, error: &str) -> Value {
    json!({
        "duration_ms": duration_ms,
        "error_len": error.len(),
        "error_sha256": sha256_hex(error),
        "error_preview": preview(error),
    })
}
