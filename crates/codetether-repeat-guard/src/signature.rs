//! Canonical tool-input signature generation.
use super::filter::filter_volatile;
use serde_json::Value;

pub(super) fn signature(tool_name: &str, input: &Value) -> String {
    let canonical = canonicalize(input);
    format!("{tool_name}:{canonical}")
}

/// Produce a stable string key from tool input, ignoring volatile fields
/// like `tool_call_id` or `instruction` that change per call.
fn canonicalize(input: &Value) -> String {
    let filtered = filter_volatile(input);
    serde_json::to_string(&filtered).unwrap_or_else(|_| input.to_string())
}
