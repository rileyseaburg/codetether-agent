//! Configured session-directory resolution using the canonical data owner.
use anyhow::Result;
use std::path::PathBuf;
/// Resolve the sessions directory without creating it.
/// # Returns
/// The sessions child of the canonical data directory, without creating it.
/// # Errors
/// Returns an error when no data directory can be determined.
/// # Examples
/// ```rust,no_run
/// let directory = codetether_session_location::sessions_dir().unwrap();
/// assert!(directory.ends_with("sessions"));
/// ```
pub fn sessions_dir() -> Result<PathBuf> {
    crate::data_dir()
        .map(|directory| directory.join("sessions"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
}
