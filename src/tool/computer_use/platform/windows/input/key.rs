//! Native key press via Win32 SendInput.

use crate::platform::windows::computer_use::{parse_send_keys, send_chord};
use crate::tool::computer_use::input::ComputerUseInput;

/// Press a key using native Win32 SendInput with proper chord handling.
pub async fn handle_press_key(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let key = input
        .key
        .as_deref()
        .or(input.text.as_deref())
        .unwrap_or("ENTER");
    let vks = parse_send_keys(key);
    send_chord(&vks)?;
    Ok(super::super::response::success_result(serde_json::json!({
        "pressed": key
    })))
}
