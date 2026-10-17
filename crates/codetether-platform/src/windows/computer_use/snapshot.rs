//! Full-resolution virtual-desktop capture in physical pixels.

use super::{dpi::DpiContext, encode::bgra_to_png, gdi_capture::capture_pixels};
use windows::Win32::UI::WindowsAndMessaging::*;

/// Captures the full virtual screen without resizing.
///
/// # Arguments
///
/// No arguments; captures the current virtual desktop.
///
/// # Returns
///
/// `(png_bytes, width, height, virtual_x, virtual_y)` in physical pixels.
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
/// use codetether_agent::platform::windows::computer_use::capture_screenshot;
/// let (png, width, height, _, _) = capture_screenshot()?;
/// assert!(!png.is_empty() && width > 0 && height > 0);
/// # Ok(()) }
/// ```
pub fn capture_screenshot() -> anyhow::Result<(Vec<u8>, u32, u32, i32, i32)> {
    let _dpi = DpiContext::enter()?;
    let width = unsafe { GetSystemMetrics(SM_CXVIRTUALSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYVIRTUALSCREEN) };
    let x = unsafe { GetSystemMetrics(SM_XVIRTUALSCREEN) };
    let y = unsafe { GetSystemMetrics(SM_YVIRTUALSCREEN) };
    let pixels = capture_pixels(i64::from(width), i64::from(height), None, x, y)?;
    let png = bgra_to_png(width as u32, height as u32, pixels)?;
    Ok((png, width as u32, height as u32, x, y))
}
