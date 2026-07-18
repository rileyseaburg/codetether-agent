//! Restore one child session and its mailbox from a manifest.

use super::super::store::{self, AgentEntry};
use super::manifest::{Lifecycle, Manifest};
use anyhow::{Result, bail};

#[path = "restore/disk_entry.rs"]
mod disk_entry;
#[path = "restore/register.rs"]
mod register;
pub(super) use register::{reserved as register_reserved, resident as register};

pub(super) async fn manifest(manifest: Manifest) -> Result<usize> {
    if !manifest.supports_current_schema() || manifest.lifecycle == Lifecycle::Closed {
        return Ok(0);
    }
    let entry = disk_entry::load(&manifest).await?;
    register(manifest, entry).await?;
    Ok(1)
}

pub(super) async fn load(manifest: &Manifest) -> Result<AgentEntry> {
    if let Some(existing) = store::get(&manifest.child_session_id) {
        if existing.owner_session_id == manifest.owner_session_id {
            return Ok(existing);
        }
        bail!(
            "persisted child {} collides with another open session",
            manifest.child_session_id
        );
    }
    let session = crate::session::Session::load(&manifest.child_session_id).await?;
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
