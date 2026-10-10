use anyhow::{Result, bail};
use windows_sys::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

/// Runs the UI-thread message loop until `WM_QUIT`.
pub(super) fn pump(hwnd: HWND) -> Result<()> {
    // SAFETY: MSG is plain data; zero is a valid initial value.
    let mut message: MSG = unsafe { std::mem::zeroed() };
    loop {
        // SAFETY: message is a valid out-pointer; null HWND reads this thread's queue.
        match unsafe { GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) } {
            0 => return Ok(()),
            -1 => bail!(
                "Cannot read window messages: {}",
                std::io::Error::last_os_error()
            ),
            _ => {}
        }
        // SAFETY: message was filled by GetMessageW on this thread.
        unsafe {
            if IsDialogMessageW(hwnd, &message) == 0 {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
    }
}
