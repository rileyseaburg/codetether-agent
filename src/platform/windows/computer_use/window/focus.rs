//! Bring a window to the foreground via Win32 SetForegroundWindow.

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowTextW, IsIconic, SW_RESTORE, SetForegroundWindow, ShowWindow,
};

/// Bring window to foreground. Restores from minimized. Returns title.
pub fn bring_to_front(hwnd: i64) -> anyhow::Result<String> {
    unsafe {
        let hwnd = HWND(hwnd as *mut _);
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);
        }
        SetForegroundWindow(hwnd).ok()?;
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buf);
        Ok(String::from_utf16_lossy(&buf[..len as usize]))
    }
}
