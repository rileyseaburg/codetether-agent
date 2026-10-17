//! Application facade for owner snapshots and creation lineage.
use super::{AGENT_STORE, AgentEntry};
#[path = "store_resolve.rs"]
mod resolve;
pub(in crate::tool::agent) use resolve::for_parent as resolve_for_parent;
pub(in crate::tool::agent) fn entries_for_parent(parent: Option<&str>) -> Vec<AgentEntry> {
    AGENT_STORE.entries_for_parent(parent)
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
    AGENT_STORE.lineage_for_session(session_id)
}
#[cfg(test)]
#[path = "store_scope_tests.rs"]
mod tests;
