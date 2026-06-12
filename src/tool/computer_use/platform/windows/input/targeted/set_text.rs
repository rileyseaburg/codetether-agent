//! Focused-field text replacement via Win32 `WM_SETTEXT`.

use crate::platform::windows::computer_use::set_foreground_window_text;
use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_set_text(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let text = input.text.as_deref().unwrap_or_default();
    let applied = set_foreground_window_text(text)?;
    Ok(super::super::super::response::success_result(serde_json::json!({
        "applied": applied,
        "chars": text.chars().count()
    })))
}
