use windows_sys::Win32::{Foundation::WPARAM, UI::WindowsAndMessaging::*};

/// Restoration also invalidates selection; it never restores capture consent.
pub(super) fn invalidates(event: WPARAM) -> bool {
    matches!(
        event as u32,
        WTS_CONSOLE_CONNECT
            | WTS_CONSOLE_DISCONNECT
            | WTS_REMOTE_CONNECT
            | WTS_REMOTE_DISCONNECT
            | WTS_SESSION_LOGON
            | WTS_SESSION_LOGOFF
            | WTS_SESSION_LOCK
            | WTS_SESSION_UNLOCK
    )
}
