//! Window snapshot output assembly.

use std::path::Path;

use crate::tool::computer_use::response;

pub(super) fn window_result(
    hwnd: i64,
    path: &Path,
    size_kb: usize,
    width: u32,
    height: u32,
    cursor: Option<(i32, i32)>,
) -> anyhow::Result<crate::tool::ToolResult> {
    let bounds = crate::platform::windows::computer_use::window::window_bounds(hwnd)?;
    Ok(response::success_result(serde_json::json!({
        "captured": true,
        "mime_type": "image/png",
        "path": path.display().to_string(),
        "size_kb": size_kb,
        "width": width,
        "height": height,
        "hwnd": hwnd,
        "left": bounds.left,
        "top": bounds.top,
        "right": bounds.right,
        "bottom": bounds.bottom,
        "coordinate_space": "window_relative_pixels",
        "click_hint": "Use this hwnd with mouse actions; x/y are coordinates within this window snapshot.",
        "cursor": cursor.map(|(x, y)| serde_json::json!({"x": x, "y": y}))
    })))
}
