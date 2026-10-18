//! Durable child identity resolution from canonical ID or display name.

use crate::scan as manifest_scan;
use anyhow::Result;

/// Resolve an owner-scoped target to its durable child session ID.
///
/// # Arguments
/// `owner` selects the owner scope; `target` is a name or durable ID.
/// # Returns
/// Returns the matching durable ID, or `None`.
/// # Errors
/// Manifest discovery or reading fails.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::durable_id;
/// # async fn example() -> anyhow::Result<()> {
/// let id = durable_id(Some("parent"), "reviewer").await?;
/// # Ok(()) }
/// ```
pub async fn durable_id(owner: Option<&str>, target: &str) -> Result<Option<String>> {
    Ok(manifest_scan::find(owner, target)
        .await?
        .map(|manifest| manifest.child_session_id))
}

/// Return durable IDs whose owner is the given parent.
///
/// # Arguments
/// `parent_id` is the durable owning session ID.
/// # Returns
/// Returns all matching child IDs.
/// # Errors
/// Manifest discovery or reading fails.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::child_ids;
/// # async fn example() -> anyhow::Result<()> {
/// let children = child_ids("parent").await?;
/// # Ok(()) }
/// ```
pub async fn child_ids(parent_id: &str) -> Result<Vec<String>> {
    Ok(manifest_scan::all()
        .await?
        .into_iter()
        .filter(|manifest| manifest.owner_session_id.as_deref() == Some(parent_id))
        .map(|manifest| manifest.child_session_id)
        .collect())
}
