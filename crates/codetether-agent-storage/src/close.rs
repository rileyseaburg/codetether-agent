//! Persisted close state and durable identity existence.
use crate::{Lifecycle, manifest_io, scan as manifest_scan};
use anyhow::Result;
use codetether_agent_registry::AgentEntry;
/// Persist the closed lifecycle before removing a live child.
///
/// # Arguments
/// `name` is the display name; `entry` contains the durable session identity.
/// # Returns
/// Returns unit after the closed manifest is durable.
/// # Errors
/// Manifest path resolution, serialization, or writing fails.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::close;
/// # async fn example<S: codetether_agent_registry::Session>(entry: &codetether_agent_registry::AgentEntry<S>) -> anyhow::Result<()> {
/// close("reviewer", entry).await?;
/// # Ok(()) }
/// ```
pub async fn close<S: codetether_agent_registry::Session>(
    name: &str,
    entry: &AgentEntry<S>,
) -> Result<()> {
    manifest_io::save_lifecycle(name, entry, Lifecycle::Closed).await
}

/// Check whether an owner-scoped durable identity exists.
///
/// # Arguments
/// `owner` selects the owner scope; `name` is a display name or durable ID.
/// # Returns
/// Returns whether a matching manifest exists.
/// # Errors
/// Manifest discovery or reading fails.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::exists;
/// # async fn example() -> anyhow::Result<()> {
/// let present = exists(Some("parent"), "reviewer").await?;
/// # Ok(()) }
/// ```
pub async fn exists(owner: Option<&str>, name: &str) -> Result<bool> {
    Ok(manifest_scan::find(owner, name).await?.is_some())
}
