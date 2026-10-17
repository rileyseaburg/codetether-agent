//! Client-area origin lookup.

use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::Graphics::Gdi::ClientToScreen;

/// Return the client area's top-left point in physical screen pixels.
pub fn client_origin(hwnd: i64) -> anyhow::Result<(i32, i32)> {
    unsafe {
        let hwnd = HWND(hwnd as *mut _);
        let mut point = POINT { x: 0, y: 0 };
        ClientToScreen(hwnd, &mut point).ok()?;
        Ok((point.x, point.y))
    }
}
