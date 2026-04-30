//! Native text input via Win32 SendInput (Unicode).

use crate::platform::windows::computer_use::send_text;
use crate::tool::computer_use::input::ComputerUseInput;

/// Type text using native Win32 Unicode key events.
pub async fn handle_type_text(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let text = input.text.as_deref().unwrap_or_default();
    send_text(text)?;
    Ok(super::super::response::success_result(serde_json::json!({
        "typed": true, "length": text.len()
    })))
}
