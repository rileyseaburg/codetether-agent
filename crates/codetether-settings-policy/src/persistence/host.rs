//! Native preference state and durable storage access.
use super::Preference;
use std::future::Future;
/// Session preference fields and storage.
/// # Implementors
/// Native sessions map the preference to existing metadata fields.
pub trait Session {
    /// Update the persisted preference value before saving.
    fn set_preference(&mut self, preference: Preference, enabled: bool);
    /// Save the current session metadata.
    /// # Errors
    /// Returns the application's existing persistence failure.
    fn save(&self) -> impl Future<Output = anyhow::Result<()>>;
}
/// Interactive preference state and status output.
/// # Implementors
/// Native clients update their displayed flag and existing status line.
pub trait View {
    /// Update the interactive preference value before saving.
    fn set_preference(&mut self, preference: Preference, enabled: bool);
    /// Replace the status line with the save result.
    fn set_status(&mut self, status: String);
}
