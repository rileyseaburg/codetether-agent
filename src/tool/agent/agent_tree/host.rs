//! Snapshot projection over the application's existing live child registry.
use crate::tool::agent::store::{self, AgentEntry};
/// Owned live-registry snapshot entry for the shared tree query.
pub struct TreeEntry(AgentEntry);
impl codetether_agent_tree::Entry for TreeEntry {
    fn id(&self) -> &str {
        self.0.id()
    }
    fn name(&self) -> &str {
        &self.0.name
    }
    fn owner_session_id(&self) -> Option<&str> {
        self.0.owner_session_id.as_deref()
    }
}
/// Registry snapshot source used by application tree operations.
pub struct ApplicationHost;
impl codetether_agent_tree::Host for ApplicationHost {
    type Entry = TreeEntry;
    fn entries() -> Vec<TreeEntry> {
        store::entries_for_parent(None)
            .into_iter()
            .map(TreeEntry)
            .collect()
    }
}
