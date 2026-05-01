//! Drag handler for computer_use tool.

use crate::platform::windows::computer_use::send_drag;
use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_drag(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let (x1, y1) = super::validate::coords(input)?;
    let x2 = super::validate::coord(input.x2, "x2")?;
    let y2 = super::validate::coord(input.y2, "y2")?;
    send_drag(x1, y1, x2, y2)?;
    Ok(super::super::response::success_result(serde_json::json!({
        "dragged": true, "x1": x1, "y1": y1, "x2": x2, "y2": y2
    })))
}
