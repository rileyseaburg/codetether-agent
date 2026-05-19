//! Native window enumeration via Win32.

use crate::platform::windows::computer_use::list_windows;

/// List visible, non-minimized windows with useful metadata.
///
/// Filters to only visible windows to reduce noise for agents.
pub async fn handle_list_apps() -> anyhow::Result<crate::tool::ToolResult> {
    let all = list_windows()?;
    let visible: Vec<serde_json::Value> = all
        .iter()
        .filter(|w| {
            let vis = w.get("visible").and_then(|v| v.as_bool()).unwrap_or(false);
            let title = w.get("title").and_then(|t| t.as_str()).unwrap_or("");
            let min = w
                .get("minimized")
                .and_then(|m| m.as_bool())
                .unwrap_or(false);
            vis && !title.is_empty() && !min
        })
        .cloned()
        .collect();
    Ok(super::response::success_result(serde_json::json!({
        "platform": "Windows",
        "count": visible.len(),
        "windows": visible
    })))
}
