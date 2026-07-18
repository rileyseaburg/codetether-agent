//! Durable child identity resolution from canonical ID or display name.

use super::manifest_scan;
use anyhow::Result;

/// Resolve an owner-scoped target to its durable child session ID.
pub(in crate::tool::agent) async fn durable_id(
    owner: Option<&str>,
    target: &str,
) -> Result<Option<String>> {
    Ok(manifest_scan::find(owner, target)
        .await?
        .map(|manifest| manifest.child_session_id))
}

/// Return durable IDs whose immediate parent is `parent_id`.
pub(in crate::tool::agent) async fn child_ids(parent_id: &str) -> Result<Vec<String>> {
    Ok(manifest_scan::all()
        .await?
        .into_iter()
        .filter(|manifest| manifest.owner_session_id.as_deref() == Some(parent_id))
        .map(|manifest| manifest.child_session_id)
        .collect())
}

/// Return whether a test fixture's durable spawn edge is open.
#[cfg(test)]
pub(in crate::tool::agent) async fn is_open(owner: Option<&str>, target: &str) -> Result<bool> {
    Ok(manifest_scan::find(owner, target)
        .await?
        .is_some_and(|manifest| manifest.lifecycle == super::manifest::Lifecycle::Open))
}
