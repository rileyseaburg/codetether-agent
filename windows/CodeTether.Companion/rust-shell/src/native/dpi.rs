use super::text::failure;
use anyhow::{Result, ensure};
use windows_sys::Win32::{
    Foundation::{HWND, RECT},
    UI::{HiDpi::GetDpiForWindow, WindowsAndMessaging::*},
};

/// Sizes the window for its DPI, or to Windows' rectangle for `WM_DPICHANGED`.
pub(super) fn resize(hwnd: HWND, suggested: Option<&RECT>) -> Result<()> {
    let (x, y, width, height, flags) = match suggested {
        Some(r) => (
            r.left,
            r.top,
            r.right - r.left,
            r.bottom - r.top,
            SWP_NOZORDER | SWP_NOACTIVATE,
        ),
        None => {
            // SAFETY: hwnd is the live window on its owner thread.
            let dpi = unsafe { GetDpiForWindow(hwnd) };
            ensure!(dpi > 0, "Cannot determine window DPI");
            let scale = |v: i64| (v * i64::from(dpi) / 96) as i32;
            (
                0,
                0,
                scale(640),
                scale(500),
                SWP_NOZORDER | SWP_NOACTIVATE | SWP_NOMOVE,
            )
        }
    };
    // SAFETY: hwnd is live; no activation or z-order change is requested.
    if unsafe { SetWindowPos(hwnd, std::ptr::null_mut(), x, y, width, height, flags) } == 0 {
        return Err(failure("Cannot size companion window"));
    }
    Ok(())
}
