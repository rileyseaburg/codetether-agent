//! Screen and compatible memory device-context ownership.
use crate::Error;
use std::ptr;
use windows_sys::Win32::Graphics::Gdi::{CreateCompatibleDC, DeleteDC, GetDC, HDC, ReleaseDC};

pub(super) struct Context {
    pub(super) source: HDC,
    pub(super) target: HDC,
}
impl Context {
    pub(super) fn new() -> Result<Self, Error> {
        // SAFETY: null HWND requests the screen DC, paired with ReleaseDC.
        let source = unsafe { GetDC(ptr::null_mut()) };
        if source.is_null() {
            return Err(Error::Capture);
        }
        // SAFETY: source is live; the returned memory DC is owned here.
        let target = unsafe { CreateCompatibleDC(source) };
        let context = Self { source, target };
        if target.is_null() {
            Err(Error::Capture)
        } else {
            Ok(context)
        }
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        // SAFETY: DCs are owned here; the selected surface is dropped first.
        unsafe {
            if !self.target.is_null() {
                DeleteDC(self.target);
            }
            ReleaseDC(ptr::null_mut(), self.source);
        }
    }
}
