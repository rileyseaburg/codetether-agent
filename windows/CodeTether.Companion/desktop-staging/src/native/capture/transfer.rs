//! Copy only the exact selected-monitor rectangle; recheck around native work.
use super::{cancel, dib::Surface, gdi::Context};
use crate::{Error, Monitor, check_available, validate_selection};
use std::sync::atomic::AtomicBool;
use windows_sys::Win32::Graphics::Gdi::{BitBlt, GdiFlush, SRCCOPY};

pub(super) fn copy(
    dc: &Context,
    _surface: &Surface<'_>,
    selected: &Monitor,
    cancelled: &AtomicBool,
) -> Result<(), Error> {
    cancel::check(cancelled)?;
    validate_selection(selected)?;
    let bounds = selected.bounds();
    let [left, top, _, _] = bounds.edges();
    // SAFETY: target has a selected bounded DIB; source is the live screen DC.
    // SRCCOPY matches the reference; no virtual-desktop union is captured.
    let copied = unsafe {
        BitBlt(
            dc.target,
            0,
            0,
            bounds.width() as i32,
            bounds.height() as i32,
            dc.source,
            left,
            top,
            SRCCOPY,
        )
    };
    // SAFETY: flush this thread's writes before accessing DIB memory or clearing it.
    let flushed = unsafe { GdiFlush() };
    if copied == 0 || flushed == 0 {
        return Err(Error::Capture);
    }
    cancel::check(cancelled)?;
    check_available()?;
    validate_selection(selected)?;
    cancel::check(cancelled)
}
