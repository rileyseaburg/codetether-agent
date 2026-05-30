//! Window bounds lookup.

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;

/// Top-level window bounds in physical screen pixels.
pub struct WindowBounds {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// Return the top-left origin for a window handle.
pub fn window_bounds(hwnd: i64) -> anyhow::Result<WindowBounds> {
    unsafe {
        let hwnd = HWND(hwnd as *mut _);
        let mut rect = std::mem::zeroed();
        GetWindowRect(hwnd, &mut rect)?;
        Ok(WindowBounds {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        })
    }
}
