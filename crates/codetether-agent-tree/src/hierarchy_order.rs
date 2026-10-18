//! Stable depth-first ordering for agents keyed by their display name.
use crate::{AgentNode, HierarchyEntry};
use std::collections::HashMap;

/// Children of `parent` (`None` = top-level), sorted by name for stability.
fn children_of<E: HierarchyEntry>(
    agents: &HashMap<String, E>,
    parent: Option<&str>,
) -> Vec<String> {
    let mut names: Vec<String> = agents
        .iter()
        .filter(|(_, a)| a.parent_name() == parent)
        .map(|(k, _)| k.clone())
        .collect();
    names.sort();
    names
}

/// Recursively append `parent`'s subtree to `out` in depth-first order.
fn push_subtree<E: HierarchyEntry>(
    agents: &HashMap<String, E>,
    parent: Option<&str>,
    out: &mut Vec<AgentNode>,
) {
    for name in children_of(agents, parent) {
        let depth = agents.get(&name).map(|a| a.depth()).unwrap_or(0);
        out.push(AgentNode {
            name: name.clone(),
            depth,
        });
        push_subtree(agents, Some(&name), out);
    }
}

/// Full depth-first ordering of every spawned agent (parents before children).
///
/// # Arguments
/// * `agents` — Entries keyed by unique display name.
///
/// # Returns
/// Reachable nodes in stable name-sorted depth-first order.
///
/// # Examples
/// ```
/// # struct Entry;
/// # impl codetether_agent_tree::HierarchyEntry for Entry {
/// #     fn parent_name(&self) -> Option<&str> { None }
/// #     fn depth(&self) -> u8 { 0 }
/// # }
/// let agents = std::collections::HashMap::<String, Entry>::new();
/// assert!(codetether_agent_tree::dfs_order(&agents).is_empty());
/// ```
pub fn dfs_order<E: HierarchyEntry>(agents: &HashMap<String, E>) -> Vec<AgentNode> {
    let mut out = Vec::with_capacity(agents.len());
    push_subtree(agents, None, &mut out);
    out
}
