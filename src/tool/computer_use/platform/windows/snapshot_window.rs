//! Whole-window capture with stable physical geometry and unique artifact paths.

use crate::platform::windows::computer_use::{cursor_position, dpi::DpiContext, window};

pub(super) fn capture(hwnd: i64) -> anyhow::Result<crate::tool::ToolResult> {
    let _dpi = DpiContext::enter()?;
    let before = window::window_bounds(hwnd)?;
    let (png, width, height) = window::capture_window_png(hwnd)?;
    let after = window::window_bounds(hwnd)?;
    anyhow::ensure!((before.left, before.top, before.right, before.bottom) ==
        (after.left, after.top, after.right, after.bottom), "Window moved during capture; retry");
    let path = std::env::temp_dir().join(format!("codetether-window-{}.png", uuid::Uuid::new_v4()));
    std::fs::write(&path, &png)?;
    tracing::info!(hwnd, width, height, png_bytes = png.len(), "Window capture encoded with bounded preview");
    super::snapshot_output::window_result(hwnd, &path, &png, width, height, cursor_position().ok(), before)
}