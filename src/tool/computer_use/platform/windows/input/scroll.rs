//! Native scroll wheel via Win32 SendInput.

use crate::platform::windows::computer_use::{move_cursor, send_scroll};
use crate::tool::computer_use::input::ComputerUseInput;

/// Send scroll wheel event using native Win32 API.
pub async fn handle_scroll(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let amount = input.scroll_amount.unwrap_or(-120);
    let target = if input.x.is_some() || input.y.is_some() {
        Some(super::validate::coords(input)?)
    } else {
        None
    };
    if let Some((x, y)) = target {
        move_cursor(x, y)?;
    }
    send_scroll(amount)?;
    Ok(super::report::mouse_result(serde_json::json!({
        "scrolled": amount,
        "target": target.map(|(x, y)| serde_json::json!({"x": x, "y": y})),
        "coordinate_mode": super::validate::coordinate_mode(input)
    })))
}
