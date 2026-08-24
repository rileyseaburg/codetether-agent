//! Atomic approval claims for explicitly approved invocations.

use crate::config::Config;
use crate::runtime_policy::{RuntimeToolPolicy, ToolPolicyOutcome};
use serde_json::Value;

pub(crate) fn approved_tool_invocation_with_config(
    config: &Config,
    tool_name: &str,
    args: &Value,
) -> bool {
    if crate::session::helper::runtime::block_prior_context_from_runtime(tool_name, args).is_some()
    {
        return false;
    }
    let policy = RuntimeToolPolicy::from_config(config);
    let decision = crate::runtime_policy::invocation_decision::decide(&policy, tool_name, args);
    if matches!(decision.outcome, ToolPolicyOutcome::Deny) {
        return false;
    }
    let scope = crate::runtime_policy::invocation_scope::for_tool(tool_name, args);
    crate::runtime_policy::approval::claim(args, tool_name, scope.action, &scope.resource)
}
