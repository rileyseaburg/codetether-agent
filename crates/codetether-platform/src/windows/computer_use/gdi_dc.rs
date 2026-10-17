//! RAII ownership of source and compatible GDI device contexts.

use windows::Win32::{Foundation::HWND, Graphics::Gdi::*};

/// A source DC paired with the window required by `ReleaseDC`.
pub(super) struct SourceDc(pub HDC, Option<HWND>);

impl SourceDc {
    /// Acquire a desktop or whole-window DC; errors if acquisition fails.
    pub(super) fn new(owner: Option<HWND>) -> anyhow::Result<Self> {
        let dc = unsafe {
            match owner {
                Some(hwnd) => GetWindowDC(Some(hwnd)),
                None => GetDC(None),
            }
        };
        anyhow::ensure!(!dc.is_invalid(), "Failed to acquire capture source DC");
        Ok(Self(dc, owner))
    }
}

impl Drop for SourceDc {
    fn drop(&mut self) {
        let _ = unsafe { ReleaseDC(self.1, self.0) };
    }
}

/// A compatible memory DC released even when capture fails.
pub(super) struct MemoryDc(pub HDC);

impl MemoryDc {
    /// Create a compatible DC; errors if GDI cannot allocate it.
    pub(super) fn new(source: &SourceDc) -> anyhow::Result<Self> {
        let dc = unsafe { CreateCompatibleDC(Some(source.0)) };
        anyhow::ensure!(!dc.is_invalid(), "CreateCompatibleDC failed");
        Ok(Self(dc))
    }
}

impl Drop for MemoryDc {
    fn drop(&mut self) {
        let _ = unsafe { DeleteDC(self.0) };
    }
}
