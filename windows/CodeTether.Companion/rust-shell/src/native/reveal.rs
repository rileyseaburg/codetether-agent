use windows_sys::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

/// Shows and focuses the local controls; this never changes capture state.
pub(super) fn show(hwnd: HWND) {
    // SAFETY: hwnd is the live main window, used on its owner thread.
    unsafe {
        ShowWindow(hwnd, SW_SHOW);
        ShowWindow(hwnd, SW_RESTORE);
        SetForegroundWindow(hwnd);
    }
}

/// Hides the controls to the tray. Closing is not Stop: state is untouched.
pub(super) fn hide(hwnd: HWND) {
    // SAFETY: hwnd is the live main window, used on its owner thread.
    unsafe {
        ShowWindow(hwnd, SW_HIDE);
    }
}
