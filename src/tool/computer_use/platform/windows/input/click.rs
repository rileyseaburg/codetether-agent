//! Native mouse click via Win32 SendInput.

use crate::platform::windows::computer_use::send_click;
use crate::tool::computer_use::input::ComputerUseInput;

/// Move cursor and click using native Win32 API.
pub async fn handle_click(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let (x, y) = super::validate::coords(input)?;
    super::modifiers::with_modifiers(&input.modifiers, || send_click(x, y))?;
    Ok(super::report::mouse_result(serde_json::json!({
        "clicked": true, "x": x, "y": y,
        "modifiers": input.modifiers,
        "coordinate_mode": if input.hwnd.is_some() { "window_relative" } else { "physical_screen" }
    })))
}
