//! Spawned-agent tree: hierarchy and DFS ordering.
//!
//! Spawned agents form a tree rooted at the main chat. Each [`SpawnedAgent`]
//! records its `parent` and `depth`; this module derives the depth-first
//! traversal used by the header rail and the Tab focus ring. The recursion
//! guard lives in [`super::agent_spawn_guard`].
//!
//! [`SpawnedAgent`]: super::agent_profile::SpawnedAgent

use std::collections::HashMap;

use super::agent_profile::SpawnedAgent;

/// Maximum nesting depth. Depth 0 = child of the main chat, so this allows
/// up to five generations of agents-spawning-agents before spawns are denied.
pub const MAX_AGENT_DEPTH: u8 = 5;

/// One entry in the depth-first agent ordering.
pub struct AgentNode {
    /// Agent map key (unique name).
    pub name: String,
    /// Nesting depth (0 = child of main chat).
    pub depth: u8,
}

/// Children of `parent` (`None` = top-level), sorted by name for stability.
fn children_of(agents: &HashMap<String, SpawnedAgent>, parent: Option<&str>) -> Vec<String> {
    let mut names: Vec<String> = agents
        .iter()
        .filter(|(_, a)| a.parent.as_deref() == parent)
        .map(|(k, _)| k.clone())
        .collect();
    names.sort();
    names
}

/// Recursively append `parent`'s subtree to `out` in depth-first order.
fn push_subtree(
    agents: &HashMap<String, SpawnedAgent>,
    parent: Option<&str>,
    out: &mut Vec<AgentNode>,
) {
    for name in children_of(agents, parent) {
        let depth = agents.get(&name).map(|a| a.depth).unwrap_or(0);
        out.push(AgentNode {
            name: name.clone(),
            depth,
        });
        push_subtree(agents, Some(&name), out);
    }
}

/// Full depth-first ordering of every spawned agent (parents before children).
pub fn dfs_order(agents: &HashMap<String, SpawnedAgent>) -> Vec<AgentNode> {
    let mut out = Vec::with_capacity(agents.len());
    push_subtree(agents, None, &mut out);
    out
}
