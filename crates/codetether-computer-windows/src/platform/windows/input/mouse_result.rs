//! Staged mouse result assembly.

use crate::input::ComputerUseInput;

pub fn staged_result(
    action: &str,
    button: &str,
    target: Option<(i32, i32)>,
    input: &ComputerUseInput,
) -> codetether_tool_core::ToolResult {
    super::report::mouse_result(serde_json::json!({
        "action": action,
        "button": button,
        "target": target.map(|(x, y)| serde_json::json!({"x": x, "y": y})),
        "modifiers": input.modifiers,
        "coordinate_mode": if input.hwnd.is_some() { "window_relative" } else { "physical_screen" }
    }))
}
