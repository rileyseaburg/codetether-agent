//! Response handling for computer_use tool.

use serde_json::Value;

pub fn success_result(output: Value) -> super::ToolResult {
    let output = serde_json::to_string_pretty(&output).unwrap_or_default();
    super::ToolResult::success(output)
}

pub fn error_result(message: impl Into<String>) -> super::ToolResult {
    let error = serde_json::json!({
        "error": {
            "message": message.into(),
            "platform": std::env::consts::OS
        }
    });
    let output = serde_json::to_string_pretty(&error).unwrap_or_default();
    super::ToolResult::error(output)
}

pub fn unsupported_platform_result() -> super::ToolResult {
    error_result("Computer use is currently supported only on Windows")
}
