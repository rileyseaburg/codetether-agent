//! Validate task presence before provider discovery.

use codetether_tool_core::ToolResult;
use serde_json::json;
pub(crate) fn task(task: Option<String>) -> Result<String, ToolResult> {
    let task = match task {
        Some(t) if !t.trim().is_empty() => t,
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "go",
                "task is required for execute action",
                Some(vec!["task"]),
                Some(json!({
                    "action": "execute",
                    "task": "implement user authentication with OAuth2"
                })),
            ));
        }
    };

    Ok(task)
}
