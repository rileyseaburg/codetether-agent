//! Normalize a single string or object task entry.
use super::{input_error::task_error, task_input::TaskInput};
use codetether_tool_core::ToolResult;
use serde_json::Value;

pub(super) fn parse_task((index, value): (usize, &Value)) -> Result<TaskInput, ToolResult> {
    let default_name = format!("Task {}", index + 1);
    if let Some(instruction) = value.as_str() {
        return Ok(TaskInput {
            id: None,
            name: default_name,
            instruction: instruction.to_string(),
            specialty: None,
            needs_worktree: None,
        });
    }
    let Some(instruction) = value.get("instruction").and_then(Value::as_str) else {
        return Err(task_error(
            "INVALID_FIELD",
            &format!("tasks[{index}] must be a string or contain an instruction"),
        ));
    };
    Ok(TaskInput {
        id: value.get("id").and_then(Value::as_str).map(String::from),
        name: value
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or(&default_name)
            .to_string(),
        instruction: instruction.to_string(),
        specialty: value
            .get("specialty")
            .and_then(Value::as_str)
            .map(String::from),
        needs_worktree: value.get("needs_worktree").and_then(Value::as_bool),
    })
}
