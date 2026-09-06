//! Capture surface ownership and selection lifetime.

use super::{
    gdi_bitmap::Bitmap,
    gdi_dc::{MemoryDc, SourceDc},
};
use windows::Win32::Graphics::Gdi::*;

/// The memory DC drops before the bitmap, even if restoring selection fails.
/// This prevents deleting a still-selected bitmap on every error path.
pub(super) struct CaptureSurface {
    pub(super) memory: MemoryDc,
    pub(super) bitmap: Bitmap,
    pub(super) source: SourceDc,
    pub(super) previous: Option<HGDIOBJ>,
}

impl CaptureSurface {
    /// Create a selected capture surface with prevalidated dimensions.
    /// Errors release every resource already acquired.
    pub(super) fn new(source: SourceDc, width: i32, height: i32) -> anyhow::Result<Self> {
        let memory = MemoryDc::new(&source)?;
        let bitmap = Bitmap::new(&source, width, height)?;
        let mut surface = Self {
            memory,
            bitmap,
            source,
            previous: None,
        };
        let previous = unsafe { SelectObject(surface.memory.0, surface.bitmap.0.into()) };
        anyhow::ensure!(
            selection_succeeded(previous),
            "SelectObject capture bitmap failed"
        );
        surface.previous = Some(previous);
        Ok(surface)
    }
}

/// Both NULL and HGDI_ERROR indicate selection failure.
pub(super) fn selection_succeeded(object: HGDIOBJ) -> bool {
    !object.is_invalid()
}

impl Drop for CaptureSurface {
    fn drop(&mut self) {
        if let Some(previous) = self.previous {
            let _ = unsafe { SelectObject(self.memory.0, previous) };
        }
        // Fields drop in declaration order: DC, bitmap, source DC.
    }
}
