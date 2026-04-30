//! Native window enumeration via Win32.

use crate::platform::windows::computer_use::list_windows;

/// List visible windows.
///
/// Maintains backward-compatible `apps` field in the response schema
/// while also providing the richer `windows` detail.
pub async fn handle_list_apps() -> anyhow::Result<crate::tool::ToolResult> {
    let windows = list_windows()?;
    let apps: Vec<serde_json::Value> = windows
        .iter()
        .filter(|w| w.get("title").and_then(|t| t.as_str()).map_or(false, |s| !s.is_empty()))
        .map(|w| {
            serde_json::json!({
                "app": w.get("title").and_then(|t| t.as_str()).unwrap_or(""),
                "pid": w.get("pid").unwrap_or(&serde_json::json!(0)),
                "window_title": w.get("title").and_then(|t| t.as_str()).unwrap_or(""),
                "hwnd": w.get("hwnd").unwrap_or(&serde_json::json!(0))
            })
        })
        .collect();
    Ok(super::response::success_result(serde_json::json!({
        "platform": "Windows",
        "count": apps.len(),
        "apps": apps,
        "windows": windows
    })))
}
