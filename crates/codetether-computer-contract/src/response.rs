//! Response handling for computer_use tool.

use codetether_tool_core::ToolResult;
use serde_json::Value;

/// Format a successful JSON tool response.
///
/// # Arguments
/// * `output` - JSON returned by native execution.
///
/// # Returns
/// A formatted tool result with the appropriate success flag.
/// # Examples
/// ```text
/// Use this result after native computer-use execution.
/// ```
pub fn success_result(output: Value) -> ToolResult {
    let output = serde_json::to_string_pretty(&output).unwrap_or_default();
    ToolResult::success(output)
}

/// Format a failure envelope with the current operating-system name.
///
/// # Arguments
/// * `message` - Displayable error detail.
///
/// # Returns
/// A formatted tool result with the appropriate success flag.
/// # Examples
/// ```text
/// Use this result after native computer-use execution.
/// ```
pub fn error_result(message: impl Into<String>) -> ToolResult {
    let error = serde_json::json!({
        "error": {
            "message": message.into(),
            "platform": std::env::consts::OS
        }
    });
    let output = serde_json::to_string_pretty(&error).unwrap_or_default();
    ToolResult::error(output)
}

/// Return the established unsupported-platform failure.
///
/// # Returns
/// A formatted tool result with the appropriate success flag.
/// # Examples
/// ```text
/// Use this result after native computer-use execution.
/// ```
pub fn unsupported_platform_result() -> ToolResult {
    error_result("Computer use supports Windows desktop automation and Linux desktop screenshots")
}
