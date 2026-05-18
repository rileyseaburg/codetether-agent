//! Multi-tab state for the native backend.

use super::NativePage;
use crate::browser::BrowserError;

/// Send-friendly native browser runtime state.
pub(in crate::browser::session) struct NativeRuntime {
    /// Open pages in tab order.
    pub pages: Vec<NativePage>,
    /// Index of the selected page.
    pub current: usize,
}

impl NativeRuntime {
    /// Create a runtime with one blank tab.
    pub fn new() -> Self {
        Self {
            pages: vec![NativePage::new()],
            current: 0,
        }
    }

    /// Return the selected page.
    ///
    /// # Errors
    ///
    /// Returns [`BrowserError::SessionNotStarted`] when no current page exists.
    pub fn current(&self) -> Result<&NativePage, BrowserError> {
        self.pages
            .get(self.current)
            .ok_or(BrowserError::SessionNotStarted)
    }

    /// Return the selected page mutably.
    ///
    /// # Errors
    ///
    /// Returns [`BrowserError::SessionNotStarted`] when no current page exists.
    pub fn current_mut(&mut self) -> Result<&mut NativePage, BrowserError> {
        self.pages
            .get_mut(self.current)
            .ok_or(BrowserError::SessionNotStarted)
    }

    /// Select a tab by index.
    ///
    /// # Errors
    ///
    /// Returns [`BrowserError::TabNotFound`] when `index` is out of range.
    pub fn select(&mut self, index: usize) -> Result<(), BrowserError> {
        if index >= self.pages.len() {
            return Err(BrowserError::TabNotFound(index));
        }
        self.current = index;
        Ok(())
    }

    /// Close a tab by index and clamp the selected tab.
    ///
    /// # Errors
    ///
    /// Returns [`BrowserError::TabNotFound`] when `index` is out of range.
    pub fn close(&mut self, index: usize) -> Result<(), BrowserError> {
        if index >= self.pages.len() {
            return Err(BrowserError::TabNotFound(index));
        }
        self.pages.remove(index);
        self.current = self
            .current
            .saturating_sub((index <= self.current) as usize);
        self.current = self.current.min(self.pages.len().saturating_sub(1));
        Ok(())
    }
}
