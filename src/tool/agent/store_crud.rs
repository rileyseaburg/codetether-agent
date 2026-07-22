//! Canonical-ID CRUD operations for the open child registry.

use super::{AGENT_STORE, AgentEntry};
use crate::session::Session;

pub(in crate::tool::agent) fn insert(entry: AgentEntry) {
    let agent_id = entry.session.id.clone();
    super::super::residency::touch(&agent_id);
    insert_reserved(entry);
}

/// Insert a child already represented by a pending residency slot.
pub(in crate::tool::agent) fn insert_reserved(entry: AgentEntry) {
    AGENT_STORE.write().insert(entry.session.id.clone(), entry);
}

pub(in crate::tool::agent) fn remove(id: &str) -> Option<AgentEntry> {
    let removed = AGENT_STORE.write().remove(id);
    super::super::residency::forget(id);
    removed
}

pub(in crate::tool::agent) fn get(id: &str) -> Option<AgentEntry> {
    AGENT_STORE.read().get(id).cloned()
}

pub(in crate::tool::agent) fn contains_name(name: &str, owner: Option<&str>) -> bool {
    AGENT_STORE.read().values().any(|entry| {
        entry.name == name && (owner.is_none() || entry.owner_session_id.as_deref() == owner)
    })
}

/// Return whether an exact owner scope already contains the display name.
pub(in crate::tool::agent) fn contains_name_for_owner(name: &str, owner: Option<&str>) -> bool {
    AGENT_STORE
        .read()
        .values()
        .any(|entry| entry.name == name && entry.owner_session_id.as_deref() == owner)
}

pub(in crate::tool::agent) fn update_session(id: &str, session: Session) {
    if let Some(entry) = AGENT_STORE.write().get_mut(id) {
        entry.session = session;
    }
}

pub(in crate::tool::agent) fn reparent_owner(from: &str, to: &str) {
    for entry in AGENT_STORE.write().values_mut() {
        if entry.owner_session_id.as_deref() == Some(from) {
            entry.owner_session_id = Some(to.to_string());
        }
    }
}
