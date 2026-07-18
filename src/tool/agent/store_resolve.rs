//! Parent-scoped lookup by canonical ID or compatible display name.

use super::super::{AGENT_STORE, AgentEntry};

pub(in crate::tool::agent) fn for_parent(
    target: &str,
    parent: Option<&str>,
) -> Option<(String, AgentEntry)> {
    if let Some(parent) = parent
        && let Ok(Some(id)) =
            crate::tool::agent::collaboration_runtime::agent_tree::resolve(parent, target)
        && let Some(entry) = AGENT_STORE.read().get(&id).cloned()
    {
        return Some((id, entry));
    }
    let store = AGENT_STORE.read();
    if let Some(entry) = store.get(target)
        && (parent.is_none() || entry.owner_session_id.as_deref() == parent)
    {
        return Some((target.into(), entry.clone()));
    }
    let mut matches = store.iter().filter(|(_, entry)| {
        entry.name == target && (parent.is_none() || entry.owner_session_id.as_deref() == parent)
    });
    let first = matches
        .next()
        .map(|(id, entry)| (id.clone(), entry.clone()));
    if matches.next().is_some() {
        None
    } else {
        first
    }
}
