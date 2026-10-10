//! Selected-monitor capture orchestration; no disk, network or consent state.
mod cancel;
mod dib;
mod gdi;
mod jpeg;
mod output;
mod pixels;
mod resize;
mod transfer;
use super::dpi;
use crate::{CapturedFrame, Error, Monitor, validate_selection};
use std::sync::atomic::AtomicBool;

pub(crate) fn capture(selected: &Monitor, cancelled: &AtomicBool) -> Result<CapturedFrame, Error> {
    cancel::check(cancelled)?;
    validate_selection(selected)?;
    let dpi = dpi::Guard::enter()?;
    let result = capture_available(selected, cancelled);
    dpi.restore()?;
    let frame = result?;
    cancel::check(cancelled)?;
    validate_selection(selected)?;
    cancel::check(cancelled)?;
    Ok(frame)
}

fn capture_available(selected: &Monitor, cancelled: &AtomicBool) -> Result<CapturedFrame, Error> {
    let pixels = {
        let dc = gdi::Context::new()?;
        let mut surface = dib::Surface::new(&dc, selected.bounds())?;
        transfer::copy(&dc, &surface, selected, cancelled)?;
        let pixels = resize::resize(&mut surface, selected.bounds(), cancelled)?;
        cancel::check(cancelled)?;
        pixels
    }; // Clear source pixels and release GDI resources before JPEG encoding.
    cancel::check(cancelled)?;
    validate_selection(selected)?;
    cancel::check(cancelled)?;
    let jpeg = jpeg::encode(&pixels, cancelled)?;
    Ok(CapturedFrame {
        jpeg,
        width: pixels.width,
        height: pixels.height,
    })
}
