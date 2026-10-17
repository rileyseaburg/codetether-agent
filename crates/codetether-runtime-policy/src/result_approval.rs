use crate::{ToolPolicyDecision, ToolPolicyOutcome, approval};
use codetether_approval_core::interaction::prefix as approval_prefix;
use codetether_tool_core::ToolResult;
use serde_json::Value;

use crate::result_approval_args as args_metadata;

pub fn blocking_result_with_approval_request(
    tool_name: &str,
    decision: ToolPolicyDecision,
    action: &str,
    resource: &str,
) -> Option<ToolResult> {
    let result = crate::blocking::blocking_result(tool_name, decision)?;
    if !matches!(decision.outcome, ToolPolicyOutcome::RequireApproval) {
        return Some(result);
    }
    Some(approval::attach_request(
        result,
        tool_name,
        action,
        resource,
        None,
        "runtime policy",
    ))
}

pub fn blocking_result_with_approval_request_for_args(
    tool_name: &str,
    decision: ToolPolicyDecision,
    action: &str,
    resource: &str,
    args: &Value,
) -> Option<ToolResult> {
    let result = crate::blocking::blocking_result(tool_name, decision)?;
    if !matches!(decision.outcome, ToolPolicyOutcome::RequireApproval) {
        return Some(result);
    }
    let amendment = approval_prefix::from_args(args);
    let result = approval::attach_request(
        result,
        tool_name,
        action,
        resource,
        amendment.as_ref(),
        args_metadata::reason(args),
    );
    Some(args_metadata::attach(result, tool_name, args))
}
