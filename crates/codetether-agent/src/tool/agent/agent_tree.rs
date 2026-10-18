//! Application snapshots for shared root-scoped agent-tree queries.

codetether_agent_tree::declare_application_tree!(
    pub(crate),
    crate::tool::agent::store::AgentEntry,
    crate::tool::agent::store::entries_for_parent,
);

#[cfg(test)]
#[path = "agent_tree/test_support.rs"]
mod test_support;
#[cfg(test)]
#[path = "agent_tree/tests.rs"]
mod tests;

#[cfg(test)]
include!("agent_tree_test_imports.rs");
