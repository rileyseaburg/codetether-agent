//! Configured runtime policy evaluation for one tool invocation.

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
    if matches!(decision.outcome, ToolPolicyOutcome::RequireApproval)
        && super::super::approval_gate::allowed(tool_name, args, scope.action, &scope.resource)
    {
        return None;
    }
    super::super::result::blocking_result_with_approval_request_for_args(
        tool_name,
        decision,
        scope.action,
        &scope.resource,
        args,
    )
}
