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
use sha2::{Digest, Sha256};

const PREVIEW_BYTES: usize = 512;

/// Detail JSON for a successful tool invocation. Always includes
/// `duration_ms` and `output_len`; `fault_code` is included when the
/// tool stamped one (typed-fault tools like `session_recall` and the
/// `context_*` family always do).
pub(in crate::session::helper) fn tool_success_detail(
    duration_ms: u64,
    result: &ToolResult,
) -> Value {
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
pub(in crate::session::helper) fn tool_failure_detail(duration_ms: u64, error: &str) -> Value {
    json!({
        "duration_ms": duration_ms,
        "error_len": error.len(),
        "error_sha256": sha256_hex(error),
        "error_preview": preview(error),
    })
}

fn sha256_hex(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    hex::encode(hasher.finalize())
}

fn preview(value: &str) -> String {
    crate::util::truncate_bytes_safe(value, PREVIEW_BYTES).to_string()
}

#[cfg(test)]
#[path = "tool_audit_detail_tests.rs"]
mod tests;
