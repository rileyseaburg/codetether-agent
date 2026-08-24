//! Configured command-prefix permission rules.

use crate::config::{PermissionAction, PermissionConfig};
use serde_json::Value;

use super::{DecisionReason, RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};

pub(super) fn outcome(permissions: &PermissionConfig, command: &str) -> Option<ToolPolicyOutcome> {
    permissions
        .rules
        .iter()
        .filter(|(prefix, _)| command.trim_start().starts_with(prefix.as_str()))
        .max_by_key(|(prefix, _)| prefix.len())
        .map(|(_, action)| match action {
            PermissionAction::Allow => ToolPolicyOutcome::Allow,
            PermissionAction::Deny => ToolPolicyOutcome::Deny,
            PermissionAction::Ask => ToolPolicyOutcome::RequireApproval,
        })
}

pub(super) fn decision(
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
