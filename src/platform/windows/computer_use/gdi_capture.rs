//! Budgeted full-resolution GDI capture orchestration.

use super::{gdi_dc::SourceDc, gdi_surface::CaptureSurface};
use crate::tool::computer_use::capture_limits::pixel_bytes;
use anyhow::Context;
use windows::Win32::{
    Foundation::HWND,
    Graphics::Gdi::{BitBlt, SRCCOPY},
};

/// Capture BGRA pixels; reject invalid/oversize dimensions before GDI allocation.
/// Returns errors for allocation, resource creation, blit, or extraction failures.
pub(super) fn capture_pixels(
    width: i64,
    height: i64,
    owner: Option<HWND>,
    x: i32,
    y: i32,
) -> anyhow::Result<Vec<u8>> {
    let byte_len = pixel_bytes(width, height)?;
    let width = i32::try_from(width).context("Capture width exceeds GDI range")?;
    let height = i32::try_from(height).context("Capture height exceeds GDI range")?;
    let mut pixels = Vec::new();
    pixels
        .try_reserve_exact(byte_len)
        .context("Cannot allocate capture pixels")?;
    pixels.resize(byte_len, 0);
    let mut surface = CaptureSurface::new(SourceDc::new(owner)?, width, height)?;
    unsafe {
        BitBlt(
            surface.memory.0,
            0,
            0,
            width,
            height,
            Some(surface.source.0),
            x,
            y,
            SRCCOPY,
        )
    }
    .context("BitBlt capture failed")?;
    surface.read_pixels(width, height, &mut pixels)?;
    Ok(pixels)
}
