use super::{DecisionReason, RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use serde_json::Value;

pub(super) fn decide(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> ToolPolicyDecision {
    if let Some(decision) = denied_command_rule(policy, tool_name, args) {
        return decision;
    }
    if let Some(decision) = super::session_command::allow(tool_name, args) {
        return decision;
    }
    if tool_name == "bash" && super::command::is_read_only_bash(args) {
        return ToolPolicyDecision::new(
            ToolPolicyOutcome::Allow,
            DecisionReason::ReadOnlyCommand,
            ToolKind::ReadOnly,
        );
    }
    if let Some(decision) = super::sandbox_preflight::decision(policy, tool_name, args) {
        return decision;
    }
    if let Some(decision) = command_rule(policy, tool_name, args) {
        return decision;
    }
    policy.decide_tool(tool_name)
}

fn denied_command_rule(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> Option<ToolPolicyDecision> {
    command_rule(policy, tool_name, args)
        .filter(|decision| matches!(decision.outcome, ToolPolicyOutcome::Deny))
}

fn command_rule(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> Option<ToolPolicyDecision> {
    let command = args.get("command").and_then(Value::as_str)?;
    let outcome = (tool_name == "bash").then(|| policy.command_rule(command))??;
    Some(ToolPolicyDecision::new(
        outcome,
        DecisionReason::MutatingTool,
        ToolKind::Mutating,
    ))
}
