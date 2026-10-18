//! Native typed binding for shared agent display ordering.
use super::agent_profile::SpawnedAgent;
pub use codetether_agent_tree::{AgentNode, MAX_AGENT_DEPTH};
use std::collections::HashMap;
/// Full depth-first ordering of every spawned agent, with parents first.
pub fn dfs_order(agents: &HashMap<String, SpawnedAgent>) -> Vec<AgentNode> {
    codetether_agent_tree::dfs_order(agents)
}
