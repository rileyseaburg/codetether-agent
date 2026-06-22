//! Focused-field text replacement via Win32 `WM_SETTEXT`.
//!
//! Some desktop controls (e.g. Blender's Font Body input) don't accept
//! `SendInput` typed characters reliably. `WM_SETTEXT` writes directly
//! into the target's text buffer and is broadcast to the control.

use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_SETTEXT};

/// Set the text of the currently focused window by posting `WM_SETTEXT`.
///
/// Returns `true` if the foreground window was found; the actual outcome
/// is best-effort because not every window accepts `WM_SETTEXT`.
pub fn set_foreground_window_text(text: &str) -> anyhow::Result<bool> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0.is_null() {
            return Ok(false);
        }
        let payload: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let lp = LPARAM(payload.as_ptr() as isize);
        let _ = SendMessageW(hwnd, WM_SETTEXT, Some(WPARAM(0)), Some(lp));
        Ok(true)
    }
}
