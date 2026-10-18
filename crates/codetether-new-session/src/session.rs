//! Native session construction, metadata and persistence operations.
use codetether_session_metadata::SessionMetadata;
use std::future::Future;
/// Existing session operations used during replacement.
/// # Implementors
/// Native session types retain their current initialization and storage behavior.
pub trait Session: Sized {
    /// Create a fresh session.
    /// # Errors
    /// Returns the native initialization failure.
    fn new() -> impl Future<Output = anyhow::Result<Self>>;
    /// Persist the current session.
    /// # Errors
    /// Returns the native storage failure.
    fn save(&self) -> impl Future<Output = anyhow::Result<()>>;
    /// Read the active session metadata.
    fn metadata(&self) -> &SessionMetadata;
    /// Update the replacement's selected metadata fields.
    fn metadata_mut(&mut self) -> &mut SessionMetadata;
    /// Read the active session identity.
    fn id(&self) -> &str;
    /// Attach the existing process bus to the replacement if needed.
    fn attach_global_bus_if_missing(&mut self);
}
