//! Tolerant discovery-record scanning and session-name resolution.

use anyhow::{Context, Result};

use super::{MuxRecord, SessionTarget};
#[path = "scan_collect.rs"]
mod collect;

/// Every server record on disk, sorted by key.
/// Load readable server records sorted by key.
/// # Arguments
/// Uses the configured global registry directory.
/// # Returns
/// Decoded records; malformed records are skipped by the existing scan policy.
/// # Errors
/// Reports registry path, scan, file-read or blocking-task failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _records = codetether_mux_registry::list().await?;
/// # Ok(()) }
/// ```
pub async fn list() -> Result<Vec<MuxRecord>> {
    let root = super::path::root()?;
    tokio::task::spawn_blocking(move || collect::run(&root))
        .await
        .context("scan mux registry")?
}

/// Resolve a globally unique session name to the server hosting it.
/// Find the server hosting a named session.
/// # Arguments
/// `name` is the validated session name.
/// # Returns
/// A target containing its record and session name, or None.
/// # Errors
/// Reports invalid names and registry scan failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _target = codetether_mux_registry::find_session("work").await?;
/// # Ok(()) }
/// ```
pub async fn find_session(name: &str) -> Result<Option<SessionTarget>> {
    super::validate_name(name)?;
    Ok(list().await?.into_iter().find_map(|record| {
        record.hosts(name).then(|| SessionTarget {
            record,
            session: name.to_string(),
        })
    }))
}

/// Resolve a session name, failing when no server hosts it.
/// Resolve a named session, requiring a hosting server.
/// # Arguments
/// `name` is the session to resolve.
/// # Returns
/// The target record and session name.
/// # Errors
/// Reports lookup failures or a missing session.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _target = codetether_mux_registry::load("work").await?;
/// # Ok(()) }
/// ```
pub async fn load(name: &str) -> Result<SessionTarget> {
    find_session(name)
        .await?
        .with_context(|| format!("mux session '{name}' was not found"))
}

/// The server already bound to `workspace`, if any record exists.
/// Find the server record for a workspace key.
/// # Arguments
/// `workspace` is the path used to derive the server key.
/// # Returns
/// The readable record, or None on any lookup failure.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let _record = codetether_mux_registry::find_workspace(std::path::Path::new("/repo")).await;
/// # }
/// ```
pub async fn find_workspace(workspace: &std::path::Path) -> Option<MuxRecord> {
    super::io::load_key(&super::key::for_workspace(workspace))
        .await
        .ok()
}
