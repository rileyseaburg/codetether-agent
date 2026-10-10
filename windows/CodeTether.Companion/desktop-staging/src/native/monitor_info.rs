use crate::{Bounds, Error, Monitor};
use std::{mem::size_of, ptr};
use windows_sys::Win32::Graphics::Gdi::*;

pub(super) fn read(handle: HMONITOR) -> Result<Monitor, Error> {
    let mut info = MONITORINFOEXW::default();
    info.monitorInfo.cbSize = size_of::<MONITORINFOEXW>() as u32;
    // SAFETY: MONITORINFOEXW begins with MONITORINFO; cbSize describes the storage.
    if unsafe { GetMonitorInfoW(handle, ptr::from_mut(&mut info).cast()) } == 0 {
        return Err(Error::Selection);
    }
    let end = info
        .szDevice
        .iter()
        .position(|&unit| unit == 0)
        .ok_or(Error::Selection)?;
    if end == 0 {
        return Err(Error::Selection);
    }
    let device = String::from_utf16(&info.szDevice[..end]).map_err(|_| Error::Selection)?;
    let rect = info.monitorInfo.rcMonitor;
    Ok(Monitor {
        device,
        bounds: Bounds::new(rect.left, rect.top, rect.right, rect.bottom)?,
        primary: info.monitorInfo.dwFlags & 1 != 0,
    })
}
