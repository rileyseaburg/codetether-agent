//! Session-scoped command-prefix decisions.

use super::{DecisionReason, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use serde_json::Value;

pub(super) fn allow(tool_name: &str, args: &Value) -> Option<ToolPolicyDecision> {
    let command = command(tool_name, args)?;
    approved_command(command).then(|| {
        ToolPolicyDecision::new(
            ToolPolicyOutcome::Allow,
            DecisionReason::SessionApproval,
            ToolKind::Mutating,
        )
    })
}

pub(super) fn approved(tool_name: &str, args: &Value) -> bool {
    command(tool_name, args).is_some_and(approved_command)
}

fn command<'a>(tool_name: &str, args: &'a Value) -> Option<&'a str> {
    super::command::value(tool_name, args)
}

fn approved_command(command: &str) -> bool {
    crate::approval::session_command_grants::allowed(command)
}
