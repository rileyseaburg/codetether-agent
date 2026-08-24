use super::{DecisionReason, RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use serde_json::Value;

pub(super) fn decide(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> ToolPolicyDecision {
    if let Some(decision) = super::policy_explicit::denial(policy, tool_name) {
        return decision;
    }
    if let Some(decision) = super::batch::decision(tool_name, args) {
        return decision;
    }
    if let Some(decision) = super::git::decision(policy, tool_name, args) {
        return decision;
    }
    if let Some(decision) = super::network::decision(tool_name, args) {
        return decision;
    }
    if let Some(decision) = super::command_rule::decision(policy, tool_name, args)
        .filter(|decision| matches!(decision.outcome, ToolPolicyOutcome::Deny))
    {
        return decision;
    }
    if let Some(decision) = super::sandbox_preflight::escalation(policy, tool_name, args) {
        return decision;
    }
    if let Some(decision) = super::session_command::allow(tool_name, args) {
        return decision;
    }
    if super::command::is_read_only_shell(tool_name, args) {
        return ToolPolicyDecision::new(
            ToolPolicyOutcome::Allow,
            DecisionReason::ReadOnlyCommand,
            ToolKind::ReadOnly,
        );
    }
    if let Some(decision) = super::command_rule::decision(policy, tool_name, args) {
        return decision;
    }
    if let Some(decision) = super::sandbox_preflight::decision(policy, tool_name, args) {
        return decision;
    }
    if let Some(decision) = super::session_transport::decision(policy, tool_name, args) {
        return decision;
    }
    policy.decide_tool(tool_name)
}
