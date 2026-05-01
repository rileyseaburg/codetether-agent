//! Native screen capture via GDI BitBlt — replaces PowerShell CopyFromScreen.

use super::screen_capture;

/// Captures the full virtual screen as PNG bytes.
///
/// Returns `(png_bytes, width, height, virtual_x, virtual_y)`.
///
/// # Errors
///
/// Returns an error if GDI bitmap creation or BitBlt fails.
pub fn capture_screenshot() -> anyhow::Result<(Vec<u8>, u32, u32, i32, i32)> {
    screen_capture::capture(image::ImageFormat::Png)
}

/// Captures the full virtual screen as JPEG bytes.
///
/// Much smaller than PNG — suitable for tool output that must
/// fit within the `tool_output_budget` (default 64 KiB).
///
/// Returns `(jpeg_bytes, width, height, virtual_x, virtual_y)`.
pub fn capture_screenshot_jpeg() -> anyhow::Result<(Vec<u8>, u32, u32, i32, i32)> {
    screen_capture::capture(image::ImageFormat::Jpeg)
}
