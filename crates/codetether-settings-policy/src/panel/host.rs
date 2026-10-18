//! Preference reads and application effects used by the settings panel.
use crate::{cycles::Cycle, persistence::Preference};
use std::future::Future;

/// Bind settings panel actions to existing application setters.
///
/// # Implementors
/// Applications read their current preference fields and retain their existing
/// persistence, status, and runtime configuration operations.
pub trait Host {
    /// Read the current value of a boolean preference.
    fn current(&self, preference: Preference) -> bool;
    /// Set and persist a boolean preference.
    fn set_preference(&mut self, preference: Preference, next: bool) -> impl Future<Output = ()>;
    /// Advance and persist the access mode.
    fn cycle_access(&mut self) -> impl Future<Output = ()>;
    /// Advance and persist the selected provider setting.
    fn cycle_provider(&mut self, setting: Cycle) -> impl Future<Output = ()>;
}
