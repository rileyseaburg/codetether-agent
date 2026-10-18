//! Transfer durable child ownership to a tail-capped continuation session.

use crate::{Host, Manifest, atomic, paths, scan as manifest_scan};
use anyhow::Result;

#[path = "migration/rollback.rs"]
mod rollback;

/// Transfer all durable children and mailboxes to a continuation owner.
///
/// # Arguments
/// `from` is the previous owner ID; `to` is the continuation owner ID.
/// # Returns
/// Returns the migrated count after updating live registry ownership.
/// # Errors
/// Discovery or migration fails; earlier migrations receive best-effort rollback.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::{Host, reparent_owner};
/// # async fn example<H: Host>() -> anyhow::Result<()> {
/// let count = reparent_owner::<H>("previous", "continuation").await?;
/// # Ok(()) }
/// ```
pub async fn reparent_owner<H: Host>(from: &str, to: &str) -> Result<usize> {
    let manifests = manifest_scan::all()
        .await?
        .into_iter()
        .filter(|item| item.owner_session_id.as_deref() == Some(from))
        .collect::<Vec<_>>();
    let mut migrated = Vec::new();
    for manifest in &manifests {
        if let Err(error) = migrate::<H>(manifest, to).await {
            rollback::run::<H>(&migrated, from, to).await;
            return Err(error);
        }
        migrated.push(manifest.clone());
    }
    H::reparent_owner(from, to);
    Ok(migrated.len())
}

async fn migrate<H: Host>(original: &Manifest, to: &str) -> Result<()> {
    let mut updated = original.clone();
    updated.owner_session_id = Some(to.to_string());
    atomic::write(&paths::manifest(&updated.child_session_id)?, &updated).await?;
    if let Err(error) = H::reparent_mailbox(
        &updated.child_session_id,
        original.owner_session_id.as_deref().unwrap_or_default(),
        to,
    )
    .await
    {
        let _ = atomic::write(&paths::manifest(&original.child_session_id)?, original).await;
        return Err(error);
    }
    Ok(())
}
