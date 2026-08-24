//! Configured runtime policy evaluation for one tool invocation.

#[path = "invocation_approved.rs"]
mod approved;
#[path = "invocation_denial.rs"]
mod denial;
#[path = "invocation_receipt.rs"]
mod receipt;
pub(super) use approved::approved_tool_invocation_with_config;
pub(super) use denial::denial_with_config;

use super::super::{RuntimeToolPolicy, ToolPolicyOutcome};
use crate::config::Config;
use crate::tool::ToolResult;
use serde_json::Value;

pub fn evaluate_tool_invocation_with_config(
    config: &Config,
    tool_name: &str,
    args: &Value,
) -> Option<ToolResult> {
    if let Some(blocked) =
        crate::session::helper::runtime::block_prior_context_from_runtime(tool_name, args)
    {
        return Some(blocked);
    }
    let policy = RuntimeToolPolicy::from_config(config);
    let decision = super::super::invocation_decision::decide(&policy, tool_name, args);
    let scope = super::super::invocation_scope::for_tool(tool_name, args);
    let receipt = args
        .get("approval_id")
        .and_then(Value::as_str)
        .is_some_and(|id| !id.trim().is_empty());
    let policy_allows = !matches!(decision.outcome, ToolPolicyOutcome::Deny);
    if policy_allows
        && (receipt || matches!(decision.outcome, ToolPolicyOutcome::RequireApproval))
        && super::super::approval_gate::allowed(tool_name, args, scope.action, &scope.resource)
    {
        return None;
    }
    if receipt && policy_allows {
        return Some(receipt::rejected(tool_name));
    }
    super::super::result::blocking_result_with_approval_request_for_args(
        tool_name,
        decision,
        scope.action,
        &scope.resource,
        args,
    )
}
