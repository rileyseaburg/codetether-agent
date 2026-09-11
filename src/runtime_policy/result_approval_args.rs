//! Approval-request metadata derived from the invocation arguments.

use crate::tool::ToolResult;
use serde_json::{Value, json};

#[path = "invocation_detail.rs"]
mod invocation_detail;
#[path = "invocation_preview.rs"]
mod invocation_preview;

/// Stored approval reason: the model's justification, else a generic label.
pub(super) fn reason(args: &Value) -> &str {
    crate::runtime_policy::justification::from_args(args).unwrap_or("runtime policy")
}

/// Attach justification, summary, and full-detail metadata for reviewers.
pub(super) fn attach(mut result: ToolResult, tool_name: &str, args: &Value) -> ToolResult {
    if let Some(text) = crate::runtime_policy::justification::from_args(args) {
        result = result.with_metadata("approval_justification", json!(text));
    }
    if let Some(preview) = invocation_preview::summarize(tool_name, args) {
        result = result.with_metadata("policy_reason", json!(preview));
    }
    if let Some(detail) = invocation_detail::render(tool_name, args) {
        result = result.with_metadata("approval_preview", json!(detail));
    }
    if crate::tool::proposed_content::supported(tool_name) {
        result = result.with_metadata("approval_arguments", args.clone());
    }
    result
}
