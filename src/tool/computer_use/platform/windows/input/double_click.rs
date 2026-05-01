//! Double-click handler for computer_use tool.

use crate::platform::windows::computer_use::send_double_click;
use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_double_click(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let (x, y) = super::validate::coords(input)?;
    send_double_click(x, y)?;
    Ok(super::super::response::success_result(serde_json::json!({
        "double_clicked": true, "x": x, "y": y
    })))
}
