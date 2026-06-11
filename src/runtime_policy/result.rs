//! Convert runtime policy decisions into tool results.

#[path = "result_approval.rs"]
mod approval_result;

pub use approval_result::blocking_result_with_approval_request;
pub(in crate::runtime_policy) use approval_result::blocking_result_with_approval_request_for_args;

use super::{RuntimeToolPolicy, ToolPolicyDecision, ToolPolicyOutcome, code};
use crate::config::Config;
use crate::tool::ToolResult;
use serde_json::json;

pub async fn evaluate_tool(tool_name: &str) -> Option<ToolResult> {
    let config = Config::load().await.unwrap_or_default();
    evaluate_tool_with_config(&config, tool_name)
}

pub fn evaluate_tool_with_config(config: &Config, tool_name: &str) -> Option<ToolResult> {
    let policy = RuntimeToolPolicy::from_config(config);
    blocking_result_with_approval_request(
        tool_name,
        policy.decide_tool(tool_name),
        "execute",
        tool_name,
    )
}

pub fn blocking_result(tool_name: &str, decision: ToolPolicyDecision) -> Option<ToolResult> {
    let (code, message) = match decision.outcome {
        ToolPolicyOutcome::Allow => return None,
        ToolPolicyOutcome::RequireApproval => (
            "TOOL_APPROVAL_REQUIRED",
            "Tool execution requires approval before it can run.",
        ),
        ToolPolicyOutcome::Deny => ("TOOL_POLICY_DENIED", "Tool execution was denied by policy."),
    };
    Some(
        ToolResult::structured_error(code, tool_name, message, None, None)
            .with_metadata("policy_outcome", json!(code::outcome(decision.outcome)))
            .with_metadata("policy_reason", json!(code::reason(decision.reason))),
    )
}
