//! Windows status handling for computer use.

pub fn handle_status() -> anyhow::Result<crate::tool::ToolResult> {
    let status = serde_json::json!({
        "supported": true,
        "platform": "Windows",
        "apis": {
            "window_enumeration": "user32 EnumWindows via PowerShell Add-Type",
            "screen_capture": "System.Drawing.CopyFromScreen",
            "input": "user32 SendInput"
        },
        "permissions": {
            "ui_access": "standard_user32_session",
            "screen_capture": "current_interactive_desktop"
        }
    });
    Ok(super::response::success_result(status))
}
