use super::text::failure;
use anyhow::Result;
use windows_sys::Win32::{Foundation::*, System::Threading::*};

pub(super) struct Instance(HANDLE);

impl Instance {
    pub(super) fn acquire() -> Result<Option<Self>> {
        // SAFETY: default security, no mutex ownership, static terminated name.
        let handle = unsafe {
            CreateMutexW(
                std::ptr::null(),
                0,
                windows_sys::w!("Local\\CodeTether.ScreenCompanion"),
            )
        };
        if handle.is_null() {
            return Err(failure("Cannot reserve companion instance"));
        }
        // SAFETY: GetLastError must immediately follow CreateMutexW.
        let exists = unsafe { GetLastError() } == ERROR_ALREADY_EXISTS;
        let instance = Self(handle);
        if exists { Ok(None) } else { Ok(Some(instance)) }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        // SAFETY: this guard owns the handle, but never owns the mutex lock.
        unsafe {
            CloseHandle(self.0);
        }
    }
}
