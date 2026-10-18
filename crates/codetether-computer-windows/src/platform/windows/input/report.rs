//! Mouse action result metadata.

use codetether_platform::windows::computer_use::{cursor_position, window::foreground_window};
use serde_json::Value;

pub fn mouse_result(mut output: Value) -> codetether_tool_core::ToolResult {
    if let Ok((x, y)) = cursor_position() {
        output["cursor_after"] = serde_json::json!({"x": x, "y": y});
    }
    if let Ok(window) = foreground_window() {
        output["foreground_window"] = window;
    }
    super::super::response::success_result(output)
}
