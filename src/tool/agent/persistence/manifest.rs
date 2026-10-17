//! Convert live application entries into shared durable manifests.
use super::super::store::AgentEntry;
pub(super) use codetether_agent_storage::{Lifecycle, Manifest};

pub(super) fn from_entry(name: &str, entry: &AgentEntry) -> Manifest {
    Manifest {
        version: 1,
        name: name.into(),
        instructions: entry.instructions.clone(),
        child_session_id: entry.session.id.clone(),
        parent: entry.parent.clone(),
        owner_session_id: entry.owner_session_id.clone(),
        depth: entry.depth,
        model_id: entry.model_id.clone(),
        lifecycle: Lifecycle::Open,
    }
}
