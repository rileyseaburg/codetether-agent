//! Native scroll wheel via Win32 SendInput.

use crate::platform::windows::computer_use::send_scroll;
use crate::tool::computer_use::input::ComputerUseInput;

/// Send scroll wheel event using native Win32 API.
pub async fn handle_scroll(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let amount = input.scroll_amount.unwrap_or(-120);
    send_scroll(amount)?;
    Ok(super::super::response::success_result(serde_json::json!({
        "scrolled": amount
    })))
}
