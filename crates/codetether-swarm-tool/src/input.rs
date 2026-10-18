//! Validate the invocation task array before normalizing each entry.
use super::task_parse::parse_task;
use super::{input_error::task_error, task_input::TaskInput};
use codetether_tool_core::ToolResult;
use serde_json::Value;

/// Parse the required non-empty task array.
/// # Arguments
/// * `params` — Tool invocation JSON.
/// # Returns
/// Tasks in input order, or a structured tool error.
/// # Errors
/// Returns a tool error for missing, empty, or invalid task entries.
/// # Examples
/// ```
/// let tasks = codetether_swarm_tool::input::parse_tasks(&serde_json::json!({"tasks":["Review"]})).unwrap();
/// assert_eq!(tasks[0].instruction, "Review");
/// ```
pub fn parse_tasks(params: &Value) -> Result<Vec<TaskInput>, ToolResult> {
    let Some(values) = params.get("tasks").and_then(Value::as_array) else {
        return Err(task_error(
            "MISSING_FIELD",
            "tasks must be a non-empty array",
        ));
    };
    if values.is_empty() {
        return Err(task_error(
            "INVALID_FIELD",
            "tasks must contain at least one task",
        ));
    }
    values.iter().enumerate().map(parse_task).collect()
}
