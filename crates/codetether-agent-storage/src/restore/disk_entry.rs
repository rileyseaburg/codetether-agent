//! Disk-restore entry loading and one-time runtime refresh registration.

use crate::{Host, Manifest};
use anyhow::Result;
use codetether_agent_registry::AgentEntry;

pub(super) async fn load<H: Host>(manifest: &Manifest) -> Result<AgentEntry<H::Session>> {
    let was_resident = H::get(&manifest.child_session_id).is_some();
    let entry = super::load::<H>(manifest).await?;
    if !was_resident {
        codetether_collaboration_state::residency::stale::mark(&manifest.child_session_id);
    }
    Ok(entry)
}
