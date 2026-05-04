//! Window enumeration via EnumWindows — replaces PowerShell Get-Process.

use serde_json::Value;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::BOOL;

/// List all top-level windows with their titles and positions.
///
/// Replaces the PowerShell `Get-Process | Where-Object MainWindowHandle`.
///
/// # Errors
///
/// Returns an error if `EnumWindows` fails catastrophically.
pub fn list_windows() -> anyhow::Result<Vec<Value>> {
    unsafe { enum_inner() }
}

unsafe fn enum_inner() -> anyhow::Result<Vec<Value>> {
    let mut results: Vec<Value> = Vec::new();
    let ptr = &mut results as *mut Vec<Value> as isize;
    unsafe { EnumWindows(Some(enum_callback), LPARAM(ptr)) }
        .ok()
        .ok_or_else(|| anyhow::anyhow!("EnumWindows failed"))?;
    Ok(results)
}

unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let results = &mut *(lparam.0 as *mut Vec<Value>);

    let mut title_buf = [0u16; 512];
    let title_len = GetWindowTextW(hwnd, &mut title_buf);
    if title_len == 0 {
        return BOOL(1); // continue
    }
    let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

    let mut rect = std::mem::zeroed();
    let _ = GetWindowRect(hwnd, &mut rect);

    let mut pid: u32 = 0;
    let _ = GetWindowThreadProcessId(hwnd, Some(&mut pid));

    results.push(serde_json::json!({
        "hwnd": hwnd.0 as i64,
        "pid": pid,
        "title": title,
        "left": rect.left,
        "top": rect.top,
        "right": rect.right,
        "bottom": rect.bottom,
    }));

    BOOL(1) // continue enumeration
}
