//! Bind shared scoped lookup to the application's live tree snapshot.
use super::super::{AGENT_STORE, AgentEntry};
pub(in crate::tool::agent) fn for_parent(
    target: &str,
    parent: Option<&str>,
) -> Option<(String, AgentEntry)> {
    AGENT_STORE.resolve_for_parent(target, parent, |parent, target| {
        crate::tool::agent::collaboration_runtime::agent_tree::resolve(parent, target)
            .ok()
            .flatten()
    })
}
