//! Foreground window inspection.

use serde_json::Value;
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowRect, GetWindowTextW, GetWindowThreadProcessId,
};

/// Return the current foreground window title, handle, process, and bounds.
pub fn foreground_window() -> anyhow::Result<Value> {
    unsafe {
        let hwnd = GetForegroundWindow();
        anyhow::ensure!(!hwnd.0.is_null(), "no foreground window");

        let mut title_buf = [0u16; 512];
        let title_len = GetWindowTextW(hwnd, &mut title_buf);
        let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

        let mut rect = std::mem::zeroed();
        GetWindowRect(hwnd, &mut rect)?;

        let mut pid = 0u32;
        let _ = GetWindowThreadProcessId(hwnd, Some(&mut pid));

        Ok(serde_json::json!({
            "hwnd": hwnd.0 as i64,
            "pid": pid,
            "title": title,
            "left": rect.left,
            "top": rect.top,
            "right": rect.right,
            "bottom": rect.bottom,
        }))
    }
}
