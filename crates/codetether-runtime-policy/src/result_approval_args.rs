//! Approval-request metadata derived from the invocation arguments.

use codetether_tool_core::ToolResult;
use serde_json::{Value, json};

#[path = "invocation_detail.rs"]
mod invocation_detail;
#[path = "invocation_preview.rs"]
mod invocation_preview;

/// Stored approval reason: the model's justification, else a generic label.
///
/// # Arguments
/// * `args` — Model-supplied invocation arguments.
/// # Returns
/// Trimmed justification or the static runtime-policy fallback.
/// # Examples
/// ```
/// assert_eq!(codetether_runtime_policy::result_approval_args::reason(&serde_json::json!({})), "runtime policy");
/// ```
pub fn reason(args: &Value) -> &str {
    crate::justification::from_args(args).unwrap_or("runtime policy")
}

/// Attach justification, summary, and full-detail metadata for reviewers.
///
/// # Arguments
/// * `result` — Existing approval-gated tool result.
/// * `tool_name` — Tool identifier for preview rendering.
/// * `args` — Original invocation arguments.
/// # Returns
/// Result enriched with reviewer metadata and arguments for supported file mutations.
/// # Examples
/// ```
/// let result = codetether_tool_core::ToolResult::success("");
/// let result = codetether_runtime_policy::result_approval_args::attach(result, "write", &serde_json::json!({"path": "note.txt"}));
/// assert_eq!(result.metadata["policy_reason"], "write file: note.txt");
/// ```
pub fn attach(mut result: ToolResult, tool_name: &str, args: &Value) -> ToolResult {
    if let Some(text) = crate::justification::from_args(args) {
        result = result.with_metadata("approval_justification", json!(text));
    }
    if let Some(preview) = invocation_preview::summarize(tool_name, args) {
        result = result.with_metadata("policy_reason", json!(preview));
    }
    if let Some(detail) = invocation_detail::render(tool_name, args) {
        result = result.with_metadata("approval_preview", json!(detail));
    }
    if codetether_tool_core::proposed::supported(tool_name) {
        result = result.with_metadata("approval_arguments", args.clone());
    }
    result
}
