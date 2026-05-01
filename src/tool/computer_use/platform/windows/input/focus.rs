//! Window focus handler for computer_use tool.

use crate::platform::windows::computer_use::bring_to_front;
use crate::tool::computer_use::input::ComputerUseInput;

/// Bring a window to the foreground by its HWND.
pub async fn handle_bring_to_front(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let hwnd = input
        .hwnd
        .ok_or_else(|| anyhow::anyhow!("hwnd is required"))?;
    let title = bring_to_front(hwnd)?;
    Ok(super::super::response::success_result(serde_json::json!({
        "focused": true, "hwnd": hwnd, "title": title
    })))
}
