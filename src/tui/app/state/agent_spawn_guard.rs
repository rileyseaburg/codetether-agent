//! Native typed bindings for shared child-depth validation.
use super::agent_profile::SpawnedAgent;
use std::collections::HashMap;

/// Depth a new child of `parent` would occupy (`None` parent → depth 0).
pub fn child_depth(agents: &HashMap<String, SpawnedAgent>, parent: Option<&str>) -> Option<u8> {
    codetether_agent_tree::child_depth(agents, parent)
}

/// Validate a spawn under `parent`, returning the child depth or an error
/// message (unknown parent or depth cap exceeded).
pub fn validate_spawn(
    agents: &HashMap<String, SpawnedAgent>,
    parent: Option<&str>,
) -> Result<u8, String> {
    codetether_agent_tree::validate_spawn(agents, parent)
}
