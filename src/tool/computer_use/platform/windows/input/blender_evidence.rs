//! Blender helper evidence capture.

use std::path::PathBuf;

use serde_json::{Value, json};

pub fn after_action(hwnd: Option<i64>) -> Value {
    match hwnd.and_then(|id| capture(id).ok()) {
        Some(evidence) => evidence,
        None => json!({
            "captured": false,
            "review_required": true,
            "path": null,
            "note": "No hwnd capture available; call window_snapshot next."
        }),
    }
}

fn capture(hwnd: i64) -> anyhow::Result<Value> {
    let (png, width, height) =
        crate::platform::windows::computer_use::window::capture_window_png(hwnd)?;
    let path = path();
    std::fs::write(&path, &png)?;
    let preview = crate::tool::computer_use::capture_preview::prepare(&png, width, height)?;
    Ok(json!({
        "captured": true,
        "image_data_url": preview.attachment,
        "preview": preview.mapping,
        "mime_type": "image/png",
        "path": path.display().to_string(),
        "size_kb": png.len() / 1024,
        "width": width,
        "height": height,
        "review_required": true,
        "selection_overlay_detected": null
    }))
}

fn path() -> PathBuf {
    std::env::temp_dir().join(format!("codetether-blender-{}.png", uuid::Uuid::new_v4()))
}