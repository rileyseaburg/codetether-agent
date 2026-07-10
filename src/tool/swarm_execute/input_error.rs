use crate::tool::ToolResult;
use serde_json::json;

pub(super) fn task_error(code: &str, message: &str) -> ToolResult {
    ToolResult::structured_error(
        code,
        "swarm_execute",
        message,
        Some(vec!["tasks"]),
        Some(json!({"tasks": ["Inspect the API", "Run focused tests"]})),
    )
}
