//! Enumerate and validate the complete monitor snapshot.
use super::callback::{Enumeration, visit};
use crate::{Error, Monitor};
use std::ptr;
use windows_sys::Win32::{Foundation::LPARAM, Graphics::Gdi::EnumDisplayMonitors};

pub(super) fn enumerate() -> Result<Vec<Monitor>, Error> {
    let mut state = Enumeration::default();
    // SAFETY: synchronous callback retains no pointers; null HDC enumerates displays.
    let ok = unsafe {
        EnumDisplayMonitors(
            ptr::null_mut(),
            ptr::null(),
            Some(visit),
            ptr::from_mut(&mut state) as LPARAM,
        )
    };
    if let Some(error) = state.error {
        return Err(error);
    }
    if ok == 0 || state.monitors.is_empty() {
        return Err(Error::Selection);
    }
    state.monitors.sort_by(|a, b| a.device.cmp(&b.device));
    if state
        .monitors
        .iter()
        .filter(|monitor| monitor.primary)
        .count()
        != 1
    {
        return Err(Error::Selection);
    }
    Ok(state.monitors)
}
