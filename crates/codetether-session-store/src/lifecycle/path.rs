//! Validated session paths independent of current worktree ownership.
use anyhow::Result;
use std::path::PathBuf;
/// Resolve the configured sessions directory.
/// # Errors
/// Returns directory-resolution failures.
pub fn sessions_dir() -> Result<PathBuf> {
    codetether_session_location::sessions_dir()
}
/// Validate a session identity and resolve its durable locator.
/// # Errors
/// Returns identity validation or location-resolution failures.
pub fn session_path(id: &str) -> Result<PathBuf> {
    crate::id::validate(id)?;
    let local = sessions_dir()?.join(format!("{id}.json"));
    codetether_session_location::resolve(id, local)
}
