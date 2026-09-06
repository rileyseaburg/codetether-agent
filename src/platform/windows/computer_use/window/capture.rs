//! Full-resolution window capture via GDI BitBlt in physical pixels.

use super::super::{dpi::DpiContext, encode::bgra_to_png, gdi_capture::capture_pixels};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;

/// Capture a window by HWND and encode its original physical extent as PNG.
///
/// # Arguments
///
/// * `hwnd` — Native window handle to capture, including its non-client area.
///
/// # Returns
///
/// `(png_bytes, width, height)` without resizing.
///
/// # Errors
///
/// Returns an error for invalid/over-budget geometry, allocation, DPI, GDI,
/// partial pixel extraction, or PNG encoding failures.
///
/// # Examples
///
/// ```rust,no_run
/// # fn main() -> anyhow::Result<()> {
/// use codetether_agent::platform::windows::computer_use::capture_window_png;
/// # let hwnd = 123_i64; // Replace with a real window handle.
/// let (png, width, height) = capture_window_png(hwnd)?;
/// assert!(!png.is_empty() && width > 0 && height > 0);
/// # Ok(()) }
/// ```
pub fn capture_window_png(hwnd: i64) -> anyhow::Result<(Vec<u8>, u32, u32)> {
    let _dpi = DpiContext::enter()?;
    let hwnd = HWND(hwnd as *mut _);
    let mut rect = RECT::default();
    unsafe { GetWindowRect(hwnd, &mut rect)? };
    let width = i64::from(rect.right) - i64::from(rect.left);
    let height = i64::from(rect.bottom) - i64::from(rect.top);
    let pixels = capture_pixels(width, height, Some(hwnd), 0, 0)?;
    let png = bgra_to_png(width as u32, height as u32, pixels)?;
    Ok((png, width as u32, height as u32))
}
