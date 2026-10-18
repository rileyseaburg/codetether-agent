//! Native HWND validation and posting using windows 0.62.2 signatures.
mod geometry;
mod observation;
mod post;
use super::types::Geometry;
use anyhow::{Result, ensure};
pub(super) use observation::observe;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetWindowThreadProcessId, IsWindow, IsWindowUnicode},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Target {
    pub hwnd: i64,
    pub process: u32,
    pub thread: u32,
}
impl Target {
    pub fn open(hwnd: i64) -> Result<Self> {
        let handle = HWND(hwnd as isize as *mut core::ffi::c_void);
        ensure!(
            unsafe { IsWindow(Some(handle)) }.as_bool(),
            "HWND is not a live window"
        );
        let mut process = 0;
        let thread = unsafe { GetWindowThreadProcessId(handle, Some(&mut process)) };
        ensure!(
            thread != 0 && process != 0,
            "Cannot identify target HWND owner"
        );
        Ok(Self {
            hwnd,
            process,
            thread,
        })
    }
    pub fn handle(self) -> HWND {
        HWND(self.hwnd as isize as *mut core::ffi::c_void)
    }
    pub fn is_unicode(self) -> bool {
        unsafe { IsWindowUnicode(self.handle()) }.as_bool()
    }
    pub fn geometry(self) -> Result<Geometry> {
        geometry::read(self.handle())
    }
}
