//! Convert client-area coordinates of a window into screen coordinates.

use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::UI::WindowsAndMessaging::{
    ClientToScreen, GetClientRect, GetForegroundWindow,
};

/// Convert a (client_x, client_y) point in the foreground window's client
/// area to absolute screen pixel coordinates. Returns `None` if the
/// foreground window is unavailable.
pub fn client_point_to_screen(client_x: i32, client_y: i32) -> Option<(i32, i32)> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0.is_null() {
            return None;
        }
        let mut _client = RECT::default();
        let _ = GetClientRect(hwnd, &mut _client);
        let mut pt = POINT {
            x: client_x,
            y: client_y,
        };
        if ClientToScreen(hwnd, &mut pt).as_bool() {
            Some((pt.x, pt.y))
        } else {
            None
        }
    }
}
