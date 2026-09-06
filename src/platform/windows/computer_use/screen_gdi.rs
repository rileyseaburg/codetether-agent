//! Extract full-resolution pixels only after restoring the memory DC selection.

use super::gdi_surface::{CaptureSurface, selection_succeeded};
use crate::tool::computer_use::capture_limits::pixel_bytes;
use windows::Win32::Graphics::Gdi::*;

impl CaptureSurface {
    /// Deselect the capture bitmap and read every requested top-down BGRA row.
    /// Returns an error for failed deselection or partial/failed extraction.
    pub(super) fn read_pixels(
        &mut self,
        width: i32,
        height: i32,
        pixels: &mut [u8],
    ) -> anyhow::Result<()> {
        let required = pixel_bytes(i64::from(width), i64::from(height))?;
        anyhow::ensure!(pixels.len() == required, "Invalid capture buffer length");
        if let Some(previous) = self.previous {
            let replaced = unsafe { SelectObject(self.memory.0, previous) };
            anyhow::ensure!(
                selection_succeeded(replaced),
                "Restore bitmap selection failed"
            );
            self.previous = None;
        }
        let mut bmi = BITMAPINFO::default();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = width;
        bmi.bmiHeader.biHeight = -height;
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        // GetDIBits requires that this bitmap is not selected into any DC.
        let rows = unsafe {
            GetDIBits(
                self.source.0,
                self.bitmap.0,
                0,
                height as u32,
                Some(pixels.as_mut_ptr().cast()),
                &mut bmi,
                DIB_RGB_COLORS,
            )
        };
        anyhow::ensure!(
            rows == height,
            "GetDIBits returned {rows} of {height} scanlines"
        );
        Ok(())
    }
}
