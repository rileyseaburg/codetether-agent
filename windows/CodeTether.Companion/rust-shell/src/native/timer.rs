use super::{ids::PULSE, text::failure};
use anyhow::Result;
use windows_sys::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{KillTimer, SetTimer},
};

pub(super) struct Timer {
    hwnd: HWND,
    id: usize,
}

impl Timer {
    pub(super) fn start(hwnd: HWND) -> Result<Self> {
        // SAFETY: notifications are posted to our window thread, without a callback.
        let id = unsafe { SetTimer(hwnd, PULSE, 500, None) };
        if id == 0 {
            return Err(failure("Cannot monitor desktop eligibility"));
        }
        Ok(Self { hwnd, id })
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        // SAFETY: this guard owns this window's timer and is dropped on its thread.
        unsafe {
            KillTimer(self.hwnd, self.id);
        }
    }
}
