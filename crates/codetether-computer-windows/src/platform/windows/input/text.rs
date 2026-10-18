//! Native text input via Win32 SendInput (Unicode).

use crate::input::ComputerUseInput;
use codetether_platform::windows::computer_use::send_text;

/// Type text using native Win32 Unicode key events.
pub async fn handle_type_text(
    input: &ComputerUseInput,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    super::keyboard_target::require(input)?;
    let text = input.text.as_deref().unwrap_or_default();
    send_text(text)?;
    Ok(super::super::response::success_result(serde_json::json!({
        "typed": true, "chars": text.chars().count()
    })))
}
