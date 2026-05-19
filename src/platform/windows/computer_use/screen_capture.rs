//! GDI-backed full-screen capture.

use super::{encode::bgra_to_encoded, screen_gdi, screen_metrics};
use windows::Win32::Graphics::Gdi::*;

/// Capture the virtual screen and encode it with the requested image format.
pub(super) fn capture(fmt: image::ImageFormat) -> anyhow::Result<(Vec<u8>, u32, u32, i32, i32)> {
    unsafe { capture_inner(fmt) }
}

unsafe fn capture_inner(fmt: image::ImageFormat) -> anyhow::Result<(Vec<u8>, u32, u32, i32, i32)> {
    let bounds = screen_metrics::virtual_screen()?;
    let hdc = unsafe { GetDC(None) };
    anyhow::ensure!(!hdc.is_invalid(), "GetDC failed");
    let mem = unsafe { CreateCompatibleDC(Some(hdc)) };
    let bm = unsafe { CreateCompatibleBitmap(hdc, bounds.width, bounds.height) };
    let old_bm = unsafe { SelectObject(mem, bm.into()) };
    let ok = unsafe {
        BitBlt(
            mem,
            0,
            0,
            bounds.width,
            bounds.height,
            Some(hdc),
            bounds.x,
            bounds.y,
            SRCCOPY,
        )
    };
    if ok.is_err() {
        unsafe { screen_gdi::cleanup(mem, bm, old_bm, hdc) };
        anyhow::bail!("BitBlt failed");
    }
    let mut bmi = screen_gdi::bitmap_info(bounds.width, bounds.height);
    let mut px = vec![0u8; (bounds.width as usize * 4) * bounds.height as usize];
    let dib_ok =
        unsafe { screen_gdi::get_pixels(mem, bm, bounds.height as u32, &mut px, &mut bmi) };
    unsafe { screen_gdi::cleanup(mem, bm, old_bm, hdc) };
    anyhow::ensure!(dib_ok != 0, "GetDIBits failed");
    let bytes = bgra_to_encoded(bounds.width as u32, bounds.height as u32, px, fmt)?;
    Ok((
        bytes,
        bounds.width as u32,
        bounds.height as u32,
        bounds.x,
        bounds.y,
    ))
}
