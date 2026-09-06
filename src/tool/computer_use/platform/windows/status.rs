//! Windows status handling for computer use.

pub fn handle_status() -> anyhow::Result<crate::tool::ToolResult> {
    let status = serde_json::json!({
        "supported": true,
        "platform": "Windows",
        "apis": {
            "window_enumeration": "native user32 EnumWindows",
            "screen_capture": "native GDI BitBlt",
            "input": "native user32 SendInput (physical mode)",
            "shadow_input": "HWND-targeted window messages; application effect is not confirmed",
            "ocr": "native WinRT Windows.Media.Ocr; use ocr_status to inspect installed recognizers"
        },
        "permissions": {
            "ui_access": "standard_user32_session",
            "screen_capture": "current_interactive_desktop"
        }
    });
    Ok(super::response::success_result(status))
}