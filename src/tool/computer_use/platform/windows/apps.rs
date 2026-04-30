//! Native window/process enumeration via Win32.

use crate::platform::windows::computer_use::{list_processes, list_windows};

/// List visible windows and running processes using native Win32 APIs.
pub async fn handle_list_apps() -> anyhow::Result<crate::tool::ToolResult> {
    let windows = list_windows()?;
    let procs = list_processes()?;
    Ok(super::response::success_result(serde_json::json!({
        "platform": "Windows",
        "windows": windows,
        "processes": procs,
        "count": windows.len()
    })))
}
