//! Native Win32 computer-use operations.
//!
//! Replaces PowerShell-mediated operations with direct Win32 API calls:
//! - Screen capture via GDI `BitBlt`
//! - Input injection via `SendInput`
//! - Window/process enumeration via `EnumWindows` / `ToolHelp32`

pub mod encode;
pub mod input;
pub mod process;
mod screen_capture;
mod screen_gdi;
mod screen_metrics;
pub mod snapshot;
pub mod window;
pub mod windows;

pub use input::{
    parse_send_keys, send_chord, send_click, send_double_click, send_drag, send_key,
    send_right_click, send_scroll, send_text,
};
pub use process::list_processes;
pub use snapshot::capture_screenshot;
pub use window::{bring_to_front, capture_window_jpeg};
pub use windows::list_windows;

/// Capture the full virtual screen as JPEG bytes.
///
/// Returns `(jpeg_bytes, width, height, virtual_x, virtual_y)`.
pub fn capture_screenshot_jpeg() -> anyhow::Result<(Vec<u8>, u32, u32, i32, i32)> {
    screen_capture::capture(image::ImageFormat::Jpeg)
}
