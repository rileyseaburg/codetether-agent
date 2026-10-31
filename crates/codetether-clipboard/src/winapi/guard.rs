//! Open the clipboard and close it on scope exit.
use windows::Win32::System::DataExchange::{CloseClipboard, OpenClipboard};

pub(super) struct ClipboardGuard;

impl ClipboardGuard {
    pub(super) fn open() -> Option<Self> {
        for attempt in 0..5 {
            if unsafe { OpenClipboard(None) }.is_ok() {
                return Some(Self);
            }
            if attempt < 4 {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
        None
    }
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseClipboard();
        }
    }
}
