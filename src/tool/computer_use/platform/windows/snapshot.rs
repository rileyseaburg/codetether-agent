//! Native screen capture — saves to temp file to avoid truncation.

use crate::platform::windows::computer_use::capture_screenshot_jpeg;

const SNAPSHOT_TMP: &str = "codetether_snapshot.jpg";
const WINDOW_SNAPSHOT_TMP: &str = "codetether_window_snapshot.jpg";

/// Capture a screenshot and save to a temp file.
///
/// Uses JPEG encoding (~50-150 KiB) instead of inline base64 PNG (~3-5 MiB)
/// so the result fits within the `tool_output_budget` (default 64 KiB).
pub async fn handle_snapshot(
    _input: &crate::tool::computer_use::input::ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let (jpeg, width, height, vx, vy) = capture_screenshot_jpeg()?;
    let path = std::env::temp_dir().join(SNAPSHOT_TMP);
    std::fs::write(&path, &jpeg)?;
    let size_kb = jpeg.len() / 1024;
    Ok(super::response::success_result(serde_json::json!({
        "captured": true,
        "mime_type": "image/jpeg",
        "path": path.display().to_string(),
        "size_kb": size_kb,
        "width": width,
        "height": height,
        "left": vx,
        "top": vy
    })))
}

/// Capture a specific window by HWND and save to a temp file.
///
/// Uses PrintWindow for window-specific capture, then crops and
/// encodes as JPEG.
pub async fn handle_window_snapshot(
    input: &crate::tool::computer_use::input::ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let hwnd = input
        .hwnd
        .ok_or_else(|| anyhow::anyhow!("hwnd is required for window_snapshot"))?;
    let (jpeg, width, height) =
        crate::platform::windows::computer_use::window::capture_window_jpeg(hwnd)?;
    let path = std::env::temp_dir().join(WINDOW_SNAPSHOT_TMP);
    std::fs::write(&path, &jpeg)?;
    let size_kb = jpeg.len() / 1024;
    Ok(super::response::success_result(serde_json::json!({
        "captured": true,
        "mime_type": "image/jpeg",
        "path": path.display().to_string(),
        "size_kb": size_kb,
        "width": width,
        "height": height,
        "hwnd": hwnd
    })))
}
