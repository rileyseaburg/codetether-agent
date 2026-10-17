//! Cursor position helpers for native computer use.

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

/// Return the current cursor position in physical screen pixels.
pub fn cursor_position() -> anyhow::Result<(i32, i32)> {
    let mut point = POINT::default();
    unsafe { GetCursorPos(&mut point)? };
    Ok((point.x, point.y))
}
