//! Runtime operations on PTY programs already owned by the registry.

use anyhow::Result;

use super::{PtyChunk, TerminalSize, registry::PtyRegistry};

impl PtyRegistry {
    /// Write bytes to a running window.
    ///
    /// # Arguments
    /// id identifies the window; data contains terminal input.
    /// # Returns
    /// Returns unit after the write.
    /// # Errors
    /// Returns an error for absent/stopped programs or failed writes.
    /// # Panics
    /// Panics if an internal mutex is poisoned.
    /// # Examples
    /// ```
    /// let registry = codetether_mux_pty::PtyRegistry::new();
    /// assert!(registry.input(1, b"hello").is_err());
    /// ```
    pub fn input(&self, id: u64, data: &[u8]) -> Result<()> {
        self.live(id)?.input(data)
    }

    /// Wait briefly for retained output or program exit.
    ///
    /// # Arguments
    /// id identifies the window; offset is the next requested output byte.
    /// # Returns
    /// Returns a bounded output chunk and its next offset.
    /// # Errors
    /// Returns an error if the window is absent.
    /// # Panics
    /// Panics if an internal mutex is poisoned.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let registry = codetether_mux_pty::PtyRegistry::new();
    /// assert!(registry.read(1, 0).await.is_err());
    /// # }
    /// ```
    pub async fn read(&self, id: u64, offset: u64) -> Result<PtyChunk> {
        Ok(self.get(id)?.read_wait(offset).await)
    }

    /// Resize the terminal of a running window.
    ///
    /// # Arguments
    /// id identifies the window; size supplies columns and rows.
    /// # Returns
    /// Returns unit after resize.
    /// # Errors
    /// Returns an error for absent/stopped programs or failed resizing.
    /// # Panics
    /// Panics if an internal mutex is poisoned.
    /// # Examples
    /// ```
    /// let registry = codetether_mux_pty::PtyRegistry::new();
    /// assert!(registry.resize(1, codetether_mux_pty::TerminalSize::new(80, 24)).is_err());
    /// ```
    pub fn resize(&self, id: u64, size: TerminalSize) -> Result<()> {
        self.live(id)?.resize(size)
    }

    /// Remove a window and signal its live process group to stop.
    ///
    /// # Panics
    /// Panics if the registry mutex is poisoned.
    pub fn stop(&self, id: u64) {
        if let Some(program) = self.programs.lock().unwrap().remove(&id) {
            program.stop();
        }
    }

    /// Signal every live process group without removing retained replay data.
    ///
    /// # Panics
    /// Panics if the registry mutex is poisoned.
    pub fn stop_all(&self) {
        for program in self.programs.lock().unwrap().values() {
            program.stop();
        }
    }
}
