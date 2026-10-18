//! Atomic writes and removal for live-entry manifests.
use crate::{Lifecycle, atomic, entry_manifest, paths};
use anyhow::Result;
use codetether_agent_registry::{AgentEntry, Session};
/// Persist an open manifest for a live entry.
///
/// # Arguments
/// `name` is the display name; `entry` supplies identity and metadata.
/// # Returns
/// Returns unit after an atomic manifest write.
/// # Errors
/// Path resolution, serialization, or writing fails.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::save;
/// # async fn example<S: codetether_agent_registry::Session>(entry: &codetether_agent_registry::AgentEntry<S>) -> anyhow::Result<()> {
/// save("reviewer", entry).await?;
/// # Ok(()) }
/// ```
pub async fn save<S: Session>(name: &str, entry: &AgentEntry<S>) -> Result<()> {
    atomic::write(
        &paths::manifest(&entry.id())?,
        &entry_manifest::from_entry(name, entry),
    )
    .await
}

/// Persist a live entry with the specified lifecycle.
///
/// # Arguments
/// `name` and `entry` supply metadata; `lifecycle` selects the persisted state.
/// # Returns
/// Returns unit after an atomic manifest write.
/// # Errors
/// Path resolution, serialization, or writing fails.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::{save_lifecycle, Lifecycle};
/// # async fn example<S: codetether_agent_registry::Session>(entry: &codetether_agent_registry::AgentEntry<S>) -> anyhow::Result<()> {
/// save_lifecycle("reviewer", entry, Lifecycle::Closed).await?;
/// # Ok(()) }
/// ```
pub async fn save_lifecycle<S: Session>(
    name: &str,
    entry: &AgentEntry<S>,
    lifecycle: Lifecycle,
) -> Result<()> {
    let manifest = entry_manifest::from_entry(name, entry).with_lifecycle(lifecycle);
    atomic::write(&paths::manifest(&entry.id())?, &manifest).await
}

/// Remove the durable manifest associated with an entry.
///
/// # Arguments
/// `entry` supplies the durable session identity.
/// # Returns
/// Returns unit when manifest removal succeeds.
/// # Errors
/// Path resolution or file removal fails.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::remove;
/// # async fn example<S: codetether_agent_registry::Session>(entry: &codetether_agent_registry::AgentEntry<S>) -> anyhow::Result<()> {
/// remove(entry).await?;
/// # Ok(()) }
/// ```
pub async fn remove<S: Session>(entry: &AgentEntry<S>) -> Result<()> {
    atomic::remove(&paths::manifest(&entry.id())?).await
}
