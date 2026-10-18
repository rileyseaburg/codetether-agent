//! Focused-field text replacement via Win32 `WM_SETTEXT`.

use crate::input::ComputerUseInput;
use codetether_platform::windows::computer_use::set_foreground_window_text;

pub async fn handle_set_text(
    input: &ComputerUseInput,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    let text = input.text.as_deref().unwrap_or_default();
    let applied = set_foreground_window_text(text)?;
    Ok(super::super::super::response::success_result(
        serde_json::json!({
            "applied": applied,
            "chars": text.chars().count()
        }),
    ))
}
