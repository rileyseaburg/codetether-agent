//! Convert a live registry entry to its version-one durable manifest.
use crate::{Lifecycle, Manifest};
use codetether_agent_registry::{AgentEntry, Session};
pub(crate) fn from_entry<S: Session>(name: &str, entry: &AgentEntry<S>) -> Manifest {
    Manifest {
        version: 1,
        name: name.into(),
        instructions: entry.instructions.clone(),
        child_session_id: entry.id().to_owned(),
        parent: entry.parent.clone(),
        owner_session_id: entry.owner_session_id.clone(),
        depth: entry.depth,
        model_id: entry.model_id.clone(),
        lifecycle: Lifecycle::Open,
    }
}
