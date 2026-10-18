//! Read-only retained-output discovery without resizing the live PTY.

use anyhow::Result;

use super::{PtyAttach, registry::PtyRegistry};

impl PtyRegistry {
    /// Read retained replay boundaries without resizing the terminal.
    ///
    /// # Arguments
    /// id identifies the window.
    /// # Returns
    /// Returns replay offsets and screen mode.
    /// # Errors
    /// Returns an error if the window is absent.
    /// # Panics
    /// Panics if an internal mutex is poisoned.
    /// # Examples
    /// ```
    /// let registry = codetether_mux_pty::PtyRegistry::new();
    /// assert!(registry.tail(1).is_err());
    /// ```
    pub fn tail(&self, id: u64) -> Result<PtyAttach> {
        Ok(self.get(id)?.attach_state())
    }
}
