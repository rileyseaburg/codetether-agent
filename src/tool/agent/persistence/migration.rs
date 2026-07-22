//! Transfer durable child ownership to a tail-capped continuation session.

use super::{atomic, manifest::Manifest, manifest_scan, paths};
use anyhow::Result;

#[path = "migration/rollback.rs"]
mod rollback;

pub(crate) async fn owner(from: &str, to: &str) -> Result<usize> {
    let manifests = manifest_scan::all()
        .await?
        .into_iter()
        .filter(|item| item.owner_session_id.as_deref() == Some(from))
        .collect::<Vec<_>>();
    let mut migrated = Vec::new();
    for manifest in &manifests {
        if let Err(error) = migrate(manifest, to).await {
            rollback::run(&migrated, from, to).await;
            return Err(error);
        }
        migrated.push(manifest.clone());
    }
    super::super::store::reparent_owner(from, to);
    Ok(migrated.len())
}

async fn migrate(original: &Manifest, to: &str) -> Result<()> {
    let mut updated = original.clone();
    updated.owner_session_id = Some(to.to_string());
    atomic::write(&paths::manifest(&updated.child_session_id)?, &updated).await?;
    if let Err(error) = super::super::collaboration_runtime::message_queue::reparent_owner(
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
