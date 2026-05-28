//! Drag handler for computer_use tool.

use crate::platform::windows::computer_use::send_drag;
use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_drag(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let (x1, y1, x2, y2) = super::validate::drag_coords(input)?;
    let button = send_drag(
        x1,
        y1,
        x2,
        y2,
        input.button.as_deref(),
        &input.modifiers,
        input.steps,
        input.duration_ms,
    )?;
    Ok(super::report::mouse_result(serde_json::json!({
        "dragged": true, "x1": x1, "y1": y1, "x2": x2, "y2": y2,
        "button": button,
        "modifiers": input.modifiers,
        "steps": input.steps,
        "duration_ms": input.duration_ms,
        "coordinate_mode": if input.hwnd.is_some() { "window_relative" } else { "physical_screen" }
    })))
}
