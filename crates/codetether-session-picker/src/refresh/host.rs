//! Session listing and picker-state access supplied by the application.
use crate::SessionPickerState;
use codetether_session_metadata::SessionSummary;
use std::{future::Future, path::Path};
/// Workspace session listing source.
/// # Implementors
/// Native sources retain their existing combined native/archive listing.
pub trait Source {
    /// List every session associated with the workspace.
    /// # Errors
    /// Returns the existing listing failure for presentation to the caller.
    fn list(cwd: &Path) -> impl Future<Output = anyhow::Result<Vec<SessionSummary>>>;
}
/// Picker state and status output for refreshing a session list.
/// # Implementors
/// Native clients provide their existing picker and status fields.
pub trait View {
    /// Borrow the picker being refreshed.
    fn picker(&mut self) -> &mut SessionPickerState;
    /// Replace the interactive status text.
    fn set_status(&mut self, status: String);
}
