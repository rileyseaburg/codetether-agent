//! Native preference state and persistence operations.
use super::Setting;
use std::future::Future;
/// Read and apply preferences through the client's existing setters.
/// # Implementors
/// Native clients retain their persistence and process-environment behavior.
pub trait Host {
    /// Read the current value from interactive state.
    fn current(&self, setting: Setting) -> bool;
    /// Apply and persist a new preference through its existing setter.
    fn apply(&mut self, setting: Setting, enabled: bool) -> impl Future<Output = ()>;
    /// Replace the interactive status line.
    fn set_status(&mut self, status: String);
}
