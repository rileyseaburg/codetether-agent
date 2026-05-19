//! Wait/delay handler for computer_use tool.

use crate::tool::computer_use::input::ComputerUseInput;

/// Wait N milliseconds for UI to settle after an action.
pub async fn handle_wait_ms(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let ms = input.ms.unwrap_or(200);
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    Ok(super::super::response::success_result(serde_json::json!({
        "waited_ms": ms
    })))
}
