use super::super::{ToolPolicyDecision, ToolPolicyOutcome, approval, approval_prefix};
use crate::tool::ToolResult;
use serde_json::Value;

#[path = "invocation_preview.rs"]
mod invocation_preview;

pub fn blocking_result_with_approval_request(
    tool_name: &str,
    decision: ToolPolicyDecision,
    action: &str,
    resource: &str,
) -> Option<ToolResult> {
    let result = super::blocking_result(tool_name, decision)?;
    if !matches!(decision.outcome, ToolPolicyOutcome::RequireApproval) {
        return Some(result);
    }
    Some(approval::attach_request(
        result, tool_name, action, resource, None,
    ))
}

pub(in crate::runtime_policy) fn blocking_result_with_approval_request_for_args(
    tool_name: &str,
    decision: ToolPolicyDecision,
    action: &str,
    resource: &str,
    args: &Value,
) -> Option<ToolResult> {
    let result = super::blocking_result(tool_name, decision)?;
    if !matches!(decision.outcome, ToolPolicyOutcome::RequireApproval) {
        return Some(result);
    }
    let amendment = approval_prefix::from_args(args);
    let mut result =
        approval::attach_request(result, tool_name, action, resource, amendment.as_ref());
    if let Some(preview) = invocation_preview::summarize(tool_name, args) {
        result = result.with_metadata("policy_reason", serde_json::json!(preview));
    }
    Some(result)
}
