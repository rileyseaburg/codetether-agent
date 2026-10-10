use super::object;
use crate::Error;
use windows_sys::Win32::System::{StationsAndDesktops::*, Threading::GetCurrentThreadId};

struct Desktop(HDESK);
impl Drop for Desktop {
    fn drop(&mut self) {
        // SAFETY: this guard owns the nonnull handle from OpenInputDesktop.
        unsafe { CloseDesktop(self.0) };
    }
}

pub(super) fn check() -> Result<(), Error> {
    // SAFETY: read-only desktop access; neither switches nor activates a desktop.
    let input = unsafe { OpenInputDesktop(0, 0, DESKTOP_READOBJECTS) };
    if input.is_null() {
        return Err(Error::Unavailable);
    }
    let input = Desktop(input);
    object::name_is(input.0, "Default")?;
    // SAFETY: thread desktop is borrowed and must not be closed by this function.
    let thread = unsafe { GetThreadDesktop(GetCurrentThreadId()) };
    object::name_is(thread, "Default")
}
