//! Session-scoped command-prefix decisions.

use super::{DecisionReason, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use serde_json::Value;

#[path = "session_command_scope.rs"]
pub(super) mod scope;

pub(super) fn allow(tool_name: &str, args: &Value) -> Option<ToolPolicyDecision> {
    let command = command(tool_name, args)?;
    approved_command(command, args).then(|| {
        ToolPolicyDecision::new(
            ToolPolicyOutcome::Allow,
            DecisionReason::SessionApproval,
            ToolKind::Mutating,
        )
    })
}

pub(super) fn approved(tool_name: &str, args: &Value) -> bool {
    command(tool_name, args).is_some_and(|command| approved_command(command, args))
}

fn command<'a>(tool_name: &str, args: &'a Value) -> Option<&'a str> {
    super::command::value(tool_name, args)
}

fn approved_command(command: &str, args: &Value) -> bool {
    let workspace = scope::from_args(args);
    !super::command_unsafe::rejected(command)
        && crate::approval::session_command_grants::allowed_scoped_in(
            command,
            args.get("__ct_session_id").and_then(Value::as_str),
            workspace.as_deref(),
        )
}

#[cfg(test)]
#[path = "session_prefix_safety_tests.rs"]
mod safety_tests;

#[cfg(test)]
#[path = "session_prefix_scope_tests.rs"]
mod scope_tests;
