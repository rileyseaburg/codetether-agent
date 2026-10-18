//! Keep native capture/encoding off the async executor's worker threads.

/// Capture the desktop on a blocking thread, retaining original pixels on disk.
pub async fn handle_snapshot(
    _input: &crate::input::ComputerUseInput,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    tokio::task::spawn_blocking(super::snapshot_desktop::capture).await?
}

/// Capture a whole window on a blocking thread with a bounded preview.
pub async fn handle_window_snapshot(
    input: &crate::input::ComputerUseInput,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    let hwnd = input
        .hwnd
        .ok_or_else(|| anyhow::anyhow!("hwnd is required for window_snapshot"))?;
    tokio::task::spawn_blocking(move || super::snapshot_window::capture(hwnd)).await?
}
