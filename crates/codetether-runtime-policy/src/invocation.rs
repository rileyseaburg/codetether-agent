//! Ordered invocation decisions with lazy host approval and sandbox lookups.

use crate::{DecisionReason, RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use serde_json::Value;

/// Apply explicit denials, session grants, read-only classification and sandbox policy.
///
/// # Arguments
/// * `policy` — Effective permission configuration.
/// * `tool_name` — Tool identifier.
/// * `args` — Invocation arguments.
/// * `session_command` — Reads session approval only after explicit denial checks.
/// * `sandbox` — Probes sandbox policy only after earlier decisions do not apply.
/// # Returns
/// The first applicable decision, otherwise the tool/action default.
/// # Examples
/// ```
/// use codetether_runtime_policy::{invocation, RuntimeToolPolicy, ToolPolicyOutcome};
/// let policy = RuntimeToolPolicy::from_config(&codetether_config::Config::default());
/// let decision = invocation::decide(&policy, "read", &serde_json::json!({}), || None, || None);
/// assert_eq!(decision.outcome, ToolPolicyOutcome::Allow);
/// ```

pub fn decide(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
    session_command: impl FnOnce() -> Option<ToolPolicyDecision>,
    sandbox: impl FnOnce() -> Option<ToolPolicyDecision>,
) -> ToolPolicyDecision {
    if let Some(decision) = super::invocation_rules::denied_command_rule(policy, tool_name, args) {
        return decision;
    }
    if let Some(decision) = session_command() {
        return decision;
    }
    if super::command::is_read_only_shell(tool_name, args) {
        return ToolPolicyDecision::new(
            ToolPolicyOutcome::Allow,
            DecisionReason::ReadOnlyCommand,
            ToolKind::ReadOnly,
        );
    }
    if let Some(decision) = super::invocation_rules::command_rule(policy, tool_name, args) {
        return decision;
    }
    if let Some(decision) = sandbox() {
        return decision;
    }
    policy.decide_tool_action(tool_name, args)
}
