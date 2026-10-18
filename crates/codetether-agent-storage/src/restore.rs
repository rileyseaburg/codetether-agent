//! Restore one child session and its mailbox from a manifest.

use crate::Host;
use crate::{Lifecycle, Manifest};
use anyhow::{Result, bail};
use codetether_agent_registry::AgentEntry;

#[path = "restore/disk_entry.rs"]
mod disk_entry;
#[path = "restore/register.rs"]
mod register;
pub(super) use register::{reserved as register_reserved, resident as register};

pub(super) async fn manifest<H: Host>(manifest: Manifest) -> Result<usize> {
    if !manifest.supports_current_schema() || manifest.lifecycle == Lifecycle::Closed {
        return Ok(0);
    }
    let entry = disk_entry::load::<H>(&manifest).await?;
    register::<H>(manifest, entry).await?;
    Ok(1)
}

pub(super) async fn load<H: Host>(manifest: &Manifest) -> Result<AgentEntry<H::Session>> {
    if let Some(existing) = H::get(&manifest.child_session_id) {
        if existing.owner_session_id == manifest.owner_session_id {
            return Ok(existing);
        }
        bail!(
            "persisted child {} collides with another open session",
            manifest.child_session_id
        );
    }
    let session = H::resume(&manifest.child_session_id).await?;
    Ok(AgentEntry {
        name: manifest.name.clone(),
        instructions: manifest.instructions.clone(),
        session,
        parent: manifest.parent.clone(),
        owner_session_id: manifest.owner_session_id.clone(),
        depth: manifest.depth,
        model_id: manifest.model_id.clone(),
    })
}
