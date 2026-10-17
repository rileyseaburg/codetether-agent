//! Assemble a configured invocation decision with a caller-supplied sandbox probe.

use crate::{RuntimeToolPolicy, ToolPolicyOutcome};
use codetether_config::Config;
use codetether_tool_core::ToolResult;
use serde_json::Value;

/// Evaluate an invocation, verifying grants or creating an approval request as needed.
///
/// # Arguments
/// * `config` — Effective runtime configuration.
/// * `tool_name` — Tool identifier.
/// * `args` — Original invocation arguments.
/// * `sandbox` — Lazy host availability probe, called only at the sandbox policy step.
/// # Returns
/// A blocking tool result, or `None` when execution may proceed.
/// # Examples
/// ```no_run
/// let config = codetether_config::Config::default();
/// let result = codetether_runtime_policy::evaluate::evaluate_tool_invocation_with_config(&config, "read", &serde_json::json!({}), |_| None);
/// assert!(result.is_none());
/// ```
pub fn evaluate_tool_invocation_with_config(
    config: &Config,
    tool_name: &str,
    args: &Value,
    sandbox: impl FnOnce(&RuntimeToolPolicy) -> Option<crate::ToolPolicyDecision>,
) -> Option<ToolResult> {
    let policy = RuntimeToolPolicy::from_config(config);
    let decision = crate::invocation::decide(
        &policy,
        tool_name,
        args,
        || crate::session_command::allow(tool_name, args),
        || sandbox(&policy),
    );
    let scope = crate::invocation_scope::for_tool(tool_name, args);
    if matches!(decision.outcome, ToolPolicyOutcome::RequireApproval)
        && crate::approval_gate::allowed(tool_name, args, scope.action, &scope.resource)
    {
        return None;
    }
    if matches!(decision.outcome, ToolPolicyOutcome::RequireApproval)
        && let Some(blocked) = crate::justification::missing(config, tool_name, decision, args)
    {
        return Some(blocked);
    }
    crate::result_approval::blocking_result_with_approval_request_for_args(
        tool_name,
        decision,
        scope.action,
        &scope.resource,
        args,
    )
}
