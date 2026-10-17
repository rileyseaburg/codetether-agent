//! Application residency hooks around shared registry mutations.
use super::{AGENT_STORE, AgentEntry};
use crate::session::Session;
pub(in crate::tool::agent) fn insert(entry: AgentEntry) {
    let agent_id = entry.session.id.clone();
    super::super::residency::touch(&agent_id);
    insert_reserved(entry);
}
/// Insert a child already represented by a pending residency slot.
pub(in crate::tool::agent) fn insert_reserved(entry: AgentEntry) {
    AGENT_STORE.insert(entry);
}
pub(in crate::tool::agent) fn remove(id: &str) -> Option<AgentEntry> {
    let removed = AGENT_STORE.remove(id);
    super::super::residency::forget(id);
    removed
}
pub(in crate::tool::agent) fn get(id: &str) -> Option<AgentEntry> {
    AGENT_STORE.get(id)
}
pub(in crate::tool::agent) fn contains_name(name: &str, owner: Option<&str>) -> bool {
    AGENT_STORE.contains_name(name, owner)
}
/// Return whether an exact owner scope already contains the display name.
pub(in crate::tool::agent) fn contains_name_for_owner(name: &str, owner: Option<&str>) -> bool {
    AGENT_STORE.contains_name_for_owner(name, owner)
}
pub(in crate::tool::agent) fn update_session(id: &str, session: Session) {
    AGENT_STORE.update_session(id, session);
}
pub(in crate::tool::agent) fn reparent_owner(from: &str, to: &str) {
    AGENT_STORE.reparent_owner(from, to);
}
