//! Disk-restore entry loading and one-time runtime refresh registration.

use super::super::manifest::Manifest;
use crate::tool::agent::store::{self, AgentEntry};
use anyhow::Result;

pub(super) async fn load(manifest: &Manifest) -> Result<AgentEntry> {
    let was_resident = store::get(&manifest.child_session_id).is_some();
    let entry = super::load(manifest).await?;
    if !was_resident {
        crate::tool::agent::residency::mark_stale(&manifest.child_session_id);
    }
    Ok(entry)
}
