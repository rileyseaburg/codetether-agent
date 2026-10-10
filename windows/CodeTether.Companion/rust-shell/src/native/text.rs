use windows_sys::Win32::UI::WindowsAndMessaging::*;

pub(super) fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(Some(0)).collect()
}

pub(crate) fn notice(message: &str) {
    let message = wide(message);
    // SAFETY: both strings are terminated and remain alive throughout the call.
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            message.as_ptr(),
            windows_sys::w!("CodeTether Screen Companion"),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}

pub(super) fn failure(context: &'static str) -> anyhow::Error {
    anyhow::anyhow!("{context}: {}", std::io::Error::last_os_error())
}
