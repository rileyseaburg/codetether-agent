//! Native key press via Win32 SendInput.

use crate::input::ComputerUseInput;
use codetether_platform::windows::computer_use::{parse_send_keys, send_chord};

/// Press a key using native Win32 SendInput with proper chord handling.
pub async fn handle_press_key(
    input: &ComputerUseInput,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    super::keyboard_target::require(input)?;
    let key = input
        .key
        .as_deref()
        .or(input.text.as_deref())
        .unwrap_or("ENTER");
    press_key_name(key)?;
    Ok(super::super::response::success_result(serde_json::json!({
        "pressed": key
    })))
}

pub fn press_key_name(key: &str) -> anyhow::Result<()> {
    let vks = parse_send_keys(key);
    send_chord(&vks)
}
