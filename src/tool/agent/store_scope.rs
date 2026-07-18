//! Parent-scoped queries over canonical child-thread identities.

use super::{AGENT_STORE, AgentEntry};

#[path = "store_resolve.rs"]
mod resolve;
pub(in crate::tool::agent) use resolve::for_parent as resolve_for_parent;

pub(in crate::tool::agent) fn entries_for_parent(parent: Option<&str>) -> Vec<AgentEntry> {
    AGENT_STORE
        .read()
        .values()
        .filter(|entry| parent.is_none() || entry.owner_session_id.as_deref() == parent)
        .cloned()
        .collect()
}

pub(in crate::tool::agent) fn get_for_parent(
    target: &str,
    parent: Option<&str>,
) -> Option<AgentEntry> {
    resolve_for_parent(target, parent).map(|(_, entry)| entry)
}

pub(in crate::tool::agent) fn lineage_for_session(
    session_id: Option<&str>,
) -> (Option<String>, u8) {
    let Some(session_id) = session_id else {
        return (None, 0);
    };
    AGENT_STORE
        .read()
        .get(session_id)
        .map_or((None, 1), |entry| {
            (Some(entry.name.clone()), entry.depth.saturating_add(1))
        })
}

#[cfg(test)]
#[path = "store_scope_tests.rs"]
mod tests;
