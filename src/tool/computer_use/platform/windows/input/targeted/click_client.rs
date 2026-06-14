//! Window-relative click via foreground client-area coordinates.

use crate::platform::windows::computer_use::{client_point_to_screen, send_click};
use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_click_client(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let x = input
        .x
        .ok_or_else(|| anyhow::anyhow!("click_client requires x"))?;
    let y = input
        .y
        .ok_or_else(|| anyhow::anyhow!("click_client requires y"))?;
    let (sx, sy) = client_point_to_screen(x as i32, y as i32)
        .ok_or_else(|| anyhow::anyhow!("foreground window unavailable for client click"))?;
    send_click(sx, sy)?;
    Ok(super::super::super::response::success_result(
        serde_json::json!({
            "clicked": true,
            "client_x": x, "client_y": y,
            "screen_x": sx, "screen_y": sy,
            "coordinate_mode": "client"
        }),
    ))
}
