//! Linux capture metadata uses image coordinates, not fabricated global geometry.

use super::{backend::Backend, image_file, session::Session};
use codetether_computer_capture as capture_preview;
use codetether_computer_contract::response;
use std::path::Path;

pub(super) fn result(
    path: &Path,
    session: Session,
    backend: Backend,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    let (png, width, height) = image_file::load(path)?;
    let preview = capture_preview::prepare(&png, width, height)?;
    Ok(response::success_result(serde_json::json!({
        "captured": true, "platform": "linux", "session_type": session.name(),
        "backend": backend.name(), "mime_type": "image/png", "path": path,
        "size_kb": png.len() / 1024, "width": width, "height": height,
        "coordinate_space": "captured_image_pixels", "cursor": null,
        "coordinate_note": "Coordinates are relative to the captured image; global desktop origin and cursor position are unavailable. Linux input injection is not supported.",
        "preview": preview.mapping
    })).with_metadata("image_data_url", preview.attachment))
}
