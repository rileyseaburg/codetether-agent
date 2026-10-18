//! Native trust-state and paused-request operations for access-mode updates.
use codetether_config::{AccessMode, Config};
/// Native actions used while changing access mode.
/// # Implementors
/// Interactive clients supply their current trust view and active session.
pub trait Host {
    /// Return the mode currently reflected by the trust view.
    fn current_mode(&self) -> AccessMode;
    /// Apply loaded configuration to trust state and the active session.
    fn apply_config(&mut self, config: &Config);
    /// Release paused requests using the existing approval mechanism.
    /// # Errors
    /// Returns the approval mechanism's existing failure message.
    fn release_all(&mut self) -> Result<usize, String>;
    /// Replace the interactive status message.
    fn set_status(&mut self, status: String);
}
