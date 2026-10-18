//! Window focus handler for computer_use tool.

use crate::input::ComputerUseInput;
use codetether_platform::windows::computer_use::bring_to_front;

/// Bring a window to the foreground by its HWND.
pub async fn handle_bring_to_front(
    input: &ComputerUseInput,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    let hwnd = input
        .hwnd
        .ok_or_else(|| anyhow::anyhow!("hwnd is required"))?;
    let title = bring_to_front(hwnd)?;
    Ok(super::super::response::success_result(serde_json::json!({
        "focused": true, "hwnd": hwnd, "title": title
    })))
}
