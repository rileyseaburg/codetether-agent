//! RAII ownership of the full-resolution capture bitmap.

use super::gdi_dc::SourceDc;
use windows::Win32::Graphics::Gdi::*;

/// Owned bitmap; its selecting DC must be released or restored before drop.
pub(super) struct Bitmap(pub HBITMAP);

impl Bitmap {
    /// Allocate a compatible bitmap using prevalidated, budgeted dimensions.
    /// Returns an error if GDI cannot allocate the bitmap.
    pub(super) fn new(source: &SourceDc, width: i32, height: i32) -> anyhow::Result<Self> {
        let bitmap = unsafe { CreateCompatibleBitmap(source.0, width, height) };
        anyhow::ensure!(!bitmap.is_invalid(), "CreateCompatibleBitmap failed");
        Ok(Self(bitmap))
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        let _ = unsafe { DeleteObject(self.0.into()) };
    }
}
