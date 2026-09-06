//! Capture and persist original desktop pixels; attach a bounded model preview.

use crate::platform::windows::computer_use::{capture_screenshot, cursor_position};
use anyhow::Context;
use crate::tool::computer_use::{capture_preview, response};

pub(super) fn capture() -> anyhow::Result<crate::tool::ToolResult> {
    let (png, width, height, left, top) = capture_screenshot()?;
    let cursor = cursor_position().ok();
    let path = std::env::temp_dir().join(format!("codetether-snapshot-{}.png", uuid::Uuid::new_v4()));
    std::fs::write(&path, &png)?;
    let preview = capture_preview::prepare(&png, width, height)
        .with_context(|| format!("Original capture saved at {}; preview failed", path.display()))?;
    tracing::info!(width, height, png_bytes = png.len(), "Desktop capture encoded with bounded preview");
    Ok(response::success_result(serde_json::json!({
        "captured": true, "mime_type": "image/png", "path": path,
        "size_kb": png.len() / 1024, "width": width, "height": height,
        "left": left, "top": top, "coordinate_space": "physical_screen_pixels",
        "preview": preview.mapping,
        "cursor": cursor.map(|(x,y)| serde_json::json!({"x":x,"y":y}))
    })).with_metadata("image_data_url", preview.attachment))
}