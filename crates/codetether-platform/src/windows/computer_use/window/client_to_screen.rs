//! Convert client-area coordinates of a window into screen coordinates.

use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

/// Convert a (client_x, client_y) point in the foreground window's client
/// area to absolute screen pixel coordinates. Returns `None` if the
/// foreground window is unavailable.
pub fn client_point_to_screen(client_x: i32, client_y: i32) -> Option<(i32, i32)> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0.is_null() {
            return None;
        }
        let mut pt = POINT {
            x: client_x,
            y: client_y,
        };
        ClientToScreen(hwnd, &mut pt).ok().ok()?;
        Some((pt.x, pt.y))
    }
}
