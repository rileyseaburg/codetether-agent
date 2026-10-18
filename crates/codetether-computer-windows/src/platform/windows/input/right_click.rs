//! Right-click handler for computer_use tool.

use crate::input::ComputerUseInput;
use codetether_platform::windows::computer_use::send_right_click;

pub async fn handle_right_click(
    input: &ComputerUseInput,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    let (x, y) = super::validate::coords(input)?;
    super::modifiers::with_modifiers(&input.modifiers, || send_right_click(x, y))?;
    Ok(super::report::mouse_result(serde_json::json!({
        "right_clicked": true, "x": x, "y": y,
        "modifiers": input.modifiers,
        "coordinate_mode": super::validate::coordinate_mode(input)
    })))
}
