//! Parent-session-scoped queries over the spawned-agent store.

use super::{AGENT_STORE, AgentEntry};

pub(in crate::tool::agent) fn list_for_parent(
    parent: Option<&str>,
) -> Vec<(String, String, usize)> {
    AGENT_STORE
        .read()
        .iter()
        .filter(|(_, entry)| parent.is_none() || entry.owner_session_id.as_deref() == parent)
        .map(|(name, entry)| {
            (
                name.clone(),
                entry.instructions.clone(),
                entry.session.messages.len(),
            )
        })
        .collect()
}

pub(in crate::tool::agent) fn get_for_parent(
    name: &str,
    parent: Option<&str>,
) -> Option<AgentEntry> {
    AGENT_STORE
        .read()
        .get(name)
        .filter(|entry| parent.is_none() || entry.owner_session_id.as_deref() == parent)
        .cloned()
}
