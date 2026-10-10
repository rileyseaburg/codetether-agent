//! Bounded, panic-contained state for synchronous Windows monitor callbacks.
use super::monitor_info;
use crate::{Error, Monitor};
use std::panic::{AssertUnwindSafe, catch_unwind};
use windows_sys::Win32::{
    Foundation::{LPARAM, RECT},
    Graphics::Gdi::{HDC, HMONITOR},
};

#[derive(Default)]
pub(super) struct Enumeration {
    pub(super) monitors: Vec<Monitor>,
    pub(super) error: Option<Error>,
}

pub(super) unsafe extern "system" fn visit(
    handle: HMONITOR,
    _: HDC,
    _: *mut RECT,
    data: LPARAM,
) -> i32 {
    // SAFETY: synchronous enumeration passes our exclusive stack context.
    let state = unsafe { &mut *(data as *mut Enumeration) };
    let result = catch_unwind(AssertUnwindSafe(|| {
        if state.monitors.len() >= 64 {
            return Err(Error::Selection);
        }
        let monitor = monitor_info::read(handle)?;
        if state
            .monitors
            .iter()
            .any(|existing| existing.device.eq_ignore_ascii_case(&monitor.device))
        {
            return Err(Error::Selection);
        }
        state.monitors.push(monitor);
        Ok(())
    }));
    match result {
        Ok(Ok(())) => 1,
        Ok(Err(error)) => {
            state.error = Some(error);
            0
        }
        Err(_) => {
            state.error = Some(Error::Selection);
            0
        }
    }
}
