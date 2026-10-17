//! Persistence operations required by context-session selection.
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
/// A durable session that can be loaded directly or selected by workspace.
///
/// # Implementors
/// Implement this for the existing session type. Keep tail/window loading in
/// the persistence layer so context lookups do not load unnecessary messages.
#[async_trait]
pub trait Source: Sized + Send {
    /// Load a bounded tail for an exact session ID.
    /// # Errors
    /// Returns persistence lookup or decoding errors.
    async fn load_tail(id: &str, window: usize) -> Result<Self>;
    /// Load a bounded tail from the latest session in an optional workspace.
    /// # Errors
    /// Returns persistence lookup or decoding errors.
    async fn last_for_directory_tail(workspace: Option<&Path>, window: usize) -> Result<Self>;
    /// Load the latest full session in an optional workspace.
    /// # Errors
    /// Returns persistence lookup or decoding errors.
    async fn last_for_directory(workspace: Option<&Path>) -> Result<Self>;
}
