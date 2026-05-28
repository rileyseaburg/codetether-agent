//! Native screen capture — saves to temp file to avoid truncation.

use crate::platform::windows::computer_use::{capture_screenshot, cursor_position};

const SNAPSHOT_TMP: &str = "codetether_snapshot.png";
const WINDOW_SNAPSHOT_TMP: &str = "codetether_window_snapshot.png";

/// Capture a screenshot and save to a temp file.
///
pub async fn handle_snapshot(
    _input: &crate::tool::computer_use::input::ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let (png, width, height, vx, vy) = capture_screenshot()?;
    let cursor = cursor_position().ok();
    let path = std::env::temp_dir().join(SNAPSHOT_TMP);
    std::fs::write(&path, &png)?;
    let size_kb = png.len() / 1024;
    Ok(super::response::success_result(serde_json::json!({
        "captured": true,
        "mime_type": "image/png",
        "path": path.display().to_string(),
        "size_kb": size_kb,
        "width": width,
        "height": height,
        "left": vx,
        "top": vy,
        "coordinate_space": "physical_screen_pixels",
        "cursor": cursor.map(|(x, y)| serde_json::json!({"x": x, "y": y}))
    })))
}

/// Capture a specific window by HWND and save to a temp file.
///
/// Uses PrintWindow for window-specific capture, then crops and encodes as PNG.
pub async fn handle_window_snapshot(
    input: &crate::tool::computer_use::input::ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let hwnd = input
        .hwnd
        .ok_or_else(|| anyhow::anyhow!("hwnd is required for window_snapshot"))?;
    let (png, width, height) =
        crate::platform::windows::computer_use::window::capture_window_png(hwnd)?;
    let cursor = cursor_position().ok();
    let path = std::env::temp_dir().join(WINDOW_SNAPSHOT_TMP);
    std::fs::write(&path, &png)?;
    let size_kb = png.len() / 1024;
    Ok(super::response::success_result(serde_json::json!({
        "captured": true,
        "mime_type": "image/png",
        "path": path.display().to_string(),
        "size_kb": size_kb,
        "width": width,
        "height": height,
        "hwnd": hwnd,
        "coordinate_space": "physical_screen_pixels",
        "cursor": cursor.map(|(x, y)| serde_json::json!({"x": x, "y": y}))
    })))
}
