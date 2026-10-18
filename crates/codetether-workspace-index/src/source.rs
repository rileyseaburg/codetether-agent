//! Storage-directory boundary supplied by the application.
use anyhow::Result;
use std::path::PathBuf;
/// Resolves the directory containing session snapshots and their sidecar index.
/// # Implementors
/// Implement this using the application's existing session-directory resolver.
pub trait Source {
    /// Return the session storage directory.
    /// # Errors
    /// Propagates failures from application directory resolution.
    fn sessions_dir() -> Result<PathBuf>;
}
