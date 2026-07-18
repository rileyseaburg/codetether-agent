//! Mailbox snapshot persistence keyed by child session ID.

use super::snapshot::Snapshot;
use crate::tool::agent::persistence::{atomic, paths};
use anyhow::Result;

pub(super) async fn load(child_id: &str) -> Result<Snapshot> {
    Ok(atomic::read(&paths::mailbox(child_id)?)
        .await?
        .unwrap_or_else(|| Snapshot::new(child_id)))
}

pub(super) async fn save(snapshot: &Snapshot) -> Result<()> {
    atomic::write(&paths::mailbox(&snapshot.child_session_id)?, snapshot).await
}

pub(super) async fn remove(child_id: &str) -> Result<()> {
    atomic::remove(&paths::mailbox(child_id)?).await
}
