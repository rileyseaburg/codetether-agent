//! Virtual screen geometry for native capture.

use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
    SetProcessDPIAware,
};

/// Virtual desktop rectangle in physical pixels.
pub(super) struct ScreenBounds {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

/// Return the virtual desktop bounds after enabling DPI awareness.
pub(super) fn virtual_screen() -> anyhow::Result<ScreenBounds> {
    unsafe { SetProcessDPIAware() };
    let width = unsafe { GetSystemMetrics(SM_CXVIRTUALSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYVIRTUALSCREEN) };
    let x = unsafe { GetSystemMetrics(SM_XVIRTUALSCREEN) };
    let y = unsafe { GetSystemMetrics(SM_YVIRTUALSCREEN) };
    anyhow::ensure!(
        width > 0 && height > 0,
        "invalid virtual screen dimensions ({width}x{height})"
    );
    Ok(ScreenBounds {
        width,
        height,
        x,
        y,
    })
}
