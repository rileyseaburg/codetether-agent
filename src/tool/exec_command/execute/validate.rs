//! Argument and safety validation before command planning.

use serde_json::Value;

use super::super::input::Input;
use crate::tool::ToolResult;

pub(super) fn input(args: &Value) -> Result<Input, ToolResult> {
    let input: Input = serde_json::from_value(args.clone())
        .map_err(|error| ToolResult::error(format!("invalid exec_command input: {error}")))?;
    if input.cmd.trim().is_empty() {
        return Err(ToolResult::error("cmd must not be empty"));
    }
    if super::super::policy::unapproved_escalation(args) {
        return Err(ToolResult::error(
            "sandbox escalation requires an approved exec_command invocation",
        ));
    }
    if let Some(blocked) = crate::tool::shell_command_guard::result("exec_command", &input.cmd) {
        return Err(blocked);
    }
    Ok(input)
}
