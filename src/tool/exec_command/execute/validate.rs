//! Argument and safety validation before command planning.

use serde_json::Value;
use std::path::Path;

use super::super::input::Input;
use crate::tool::ToolResult;

pub(super) fn bind_default_workdir(mut args: Value, default: Option<&Path>) -> Value {
    if args.get("workdir").is_none()
        && let (Some(map), Some(path)) = (args.as_object_mut(), default)
    {
        map.insert("workdir".into(), path.display().to_string().into());
    }
    args
}

pub(super) async fn runtime_policy(args: &Value) -> Option<ToolResult> {
    crate::runtime_policy::evaluate_tool_invocation("exec_command", args).await
}

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
    if let Some(blocked) = crate::tool::command_workdir::result("exec_command", args) {
        return Err(blocked);
    }
    if let Some(blocked) = crate::tool::shell_command_guard::result_for_args("exec_command", args) {
        return Err(blocked);
    }
    Ok(input)
}