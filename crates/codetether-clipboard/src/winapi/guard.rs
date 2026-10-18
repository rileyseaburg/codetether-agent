//! Open the clipboard and close it on scope exit.
use windows::Win32::System::DataExchange::{CloseClipboard, OpenClipboard};

pub(super) struct ClipboardGuard;

impl ClipboardGuard {
    pub(super) fn open() -> Option<Self> {
        (0..5).find_map(|_| unsafe { OpenClipboard(None) }.ok().map(|_| Self))
    }
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseClipboard();
        }
    }
}
