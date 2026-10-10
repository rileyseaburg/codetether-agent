use crate::Error;
use std::{mem::size_of, ptr};
use windows_sys::Win32::System::{RemoteDesktop::*, Threading::*};

struct Buffer(*mut u16);
impl Drop for Buffer {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: WTS allocated this buffer; this guard is its sole owner.
            unsafe { WTSFreeMemory(self.0.cast()) };
        }
    }
}

pub(super) fn check() -> Result<(), Error> {
    let mut session = 0;
    // SAFETY: session points to valid writable storage.
    if unsafe { ProcessIdToSessionId(GetCurrentProcessId(), &mut session) } == 0 || session == 0 {
        return Err(Error::Unavailable);
    }
    let mut buffer = Buffer(ptr::null_mut());
    let mut bytes = 0;
    // SAFETY: output pointers remain valid; Buffer frees any returned allocation.
    let ok = unsafe {
        WTSQuerySessionInformationW(
            WTS_CURRENT_SERVER_HANDLE,
            session,
            WTSConnectState,
            &mut buffer.0,
            &mut bytes,
        )
    };
    if ok == 0 || buffer.0.is_null() || bytes as usize != size_of::<WTS_CONNECTSTATE_CLASS>() {
        return Err(Error::Unavailable);
    }
    // SAFETY: the successful query returned a buffer of the checked size.
    let state = unsafe { ptr::read_unaligned(buffer.0.cast::<WTS_CONNECTSTATE_CLASS>()) };
    if state == WTSActive {
        Ok(())
    } else {
        Err(Error::Unavailable)
    }
}
