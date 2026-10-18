//! Resolve durable sessions independently of the current mux worktree.
//!
//! [resolve] prefers a local file, then a saved pointer, then bounded worktree
//! discovery. [record] and [forget] manage location pointers, while [data_dir]
//! and [sessions_dir] preserve canonical storage ownership.

mod discovery;
mod registry;

use std::path::PathBuf;

use anyhow::Result;

/// Resolve a validated session ID without depending on the caller's worktree.
/// # Arguments
/// `id` is the validated session identity; `local` is its current workspace path.
/// # Returns
/// The existing local file, a valid saved/discovered location, or the local fallback.
/// # Errors
/// The signature retains the application resolver's error contract; pointer
/// lookup failures are handled through the existing fallback behavior.
/// # Examples
/// ```rust,no_run
/// # fn example(local: std::path::PathBuf) -> anyhow::Result<()> {
/// let path = codetether_session_location::resolve("session-123", local)?;
/// # Ok(()) }
/// ```
pub fn resolve(id: &str, local: PathBuf) -> Result<PathBuf> {
    if local.is_file() {
        record(id, &local);
        return Ok(local);
    }
    if let Some(path) = registry::read(id).ok().flatten() {
        return Ok(path);
    }
    let Some(path) = discovery::find(id) else {
        return Ok(local);
    };
    record(id, &path);
    Ok(path)
}

/// Record a validated session ID's location, logging pointer-write failures.
/// # Arguments
/// `id` is the validated identity; `path` is the durable session location.
/// # Returns
/// Unit; location pointer errors do not fail a successful session save.
/// # Examples
/// ```rust,no_run
/// # fn example(path: &std::path::Path) {
/// codetether_session_location::record("session-123", path);
/// # }
/// ```
pub fn record(id: &str, path: &std::path::Path) {
    if let Err(error) = registry::write(id, path) {
        tracing::warn!(target: "codetether_agent::session::persistence::location", session_id = id, %error, "session location record failed");
    }
}

/// Remove a validated session ID's saved location, logging cleanup failures.
/// # Arguments
/// `id` is the validated identity whose pointer is no longer needed.
/// # Returns
/// Unit; a missing pointer is treated as already removed.
/// # Examples
/// ```rust,no_run
/// codetether_session_location::forget("session-123");
/// ```
pub fn forget(id: &str) {
    if let Err(error) = registry::remove(id) {
        tracing::warn!(target: "codetether_agent::session::persistence::location", session_id = id, %error, "session location cleanup failed");
    }
}

mod canonical;
mod paths;
pub use canonical::data_dir;
pub use paths::sessions_dir;
