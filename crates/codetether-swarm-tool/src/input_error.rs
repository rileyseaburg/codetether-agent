//! Structured validation failures for direct swarm task input.
use codetether_tool_core::ToolResult;
use serde_json::json;

pub fn task_error(code: &str, message: &str) -> ToolResult {
    ToolResult::structured_error(
        code,
        "swarm_execute",
        message,
        Some(vec!["tasks"]),
        Some(json!({"tasks": ["Inspect the API", "Run focused tests"]})),
    )
}
