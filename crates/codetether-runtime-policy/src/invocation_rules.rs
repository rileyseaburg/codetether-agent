//! Explicit command rules used by ordered invocation decisions.

use crate::{DecisionReason, RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use serde_json::Value;

pub(super) fn denied_command_rule(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> Option<ToolPolicyDecision> {
    command_rule(policy, tool_name, args)
        .filter(|decision| matches!(decision.outcome, ToolPolicyOutcome::Deny))
}

pub(super) fn command_rule(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> Option<ToolPolicyDecision> {
    let command = super::command::value(tool_name, args)?;
    let outcome = policy.command_rule(command)?;
    Some(ToolPolicyDecision::new(
        outcome,
        DecisionReason::MutatingTool,
        ToolKind::Mutating,
    ))
}
