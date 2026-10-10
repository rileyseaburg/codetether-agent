use super::text::failure;
use anyhow::Result;
use windows_sys::Win32::{
    Foundation::HWND,
    System::RemoteDesktop::{
        NOTIFY_FOR_THIS_SESSION, WTSRegisterSessionNotification, WTSUnRegisterSessionNotification,
    },
};

pub(super) struct SessionWatch(HWND);

impl SessionWatch {
    pub(super) fn register(hwnd: HWND) -> Result<Self> {
        // SAFETY: hwnd is our live window, owned by this UI thread.
        if unsafe { WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_THIS_SESSION) } == 0 {
            return Err(failure("Cannot monitor Windows session changes"));
        }
        Ok(Self(hwnd))
    }
}

impl Drop for SessionWatch {
    fn drop(&mut self) {
        // SAFETY: this guard unregisters the window it registered, on its owner thread.
        unsafe {
            WTSUnRegisterSessionNotification(self.0);
        }
    }
}
