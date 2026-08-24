//! Terminal configured denial used before orchestration delegation.

use crate::config::Config;
use crate::runtime_policy::{RuntimeToolPolicy, ToolPolicyOutcome};
use crate::tool::ToolResult;
use serde_json::Value;

pub(crate) fn denial_with_config(
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
    let decision = crate::runtime_policy::invocation_decision::decide(&policy, tool_name, args);
    if !matches!(decision.outcome, ToolPolicyOutcome::Deny) {
        return None;
    }
    let scope = crate::runtime_policy::invocation_scope::for_tool(tool_name, args);
    crate::runtime_policy::result::blocking_result_with_approval_request_for_args(
        tool_name,
        decision,
        scope.action,
        &scope.resource,
        args,
    )
}
