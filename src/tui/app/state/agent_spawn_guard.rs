//! Recursion guard for spawning agents that spawn agents.
//!
//! Enforces [`super::agent_tree::MAX_AGENT_DEPTH`] so a runaway agent cannot
//! fork-bomb the terminal, and resolves the depth a new child would occupy.

use std::collections::HashMap;

use super::agent_profile::SpawnedAgent;
use super::agent_tree::MAX_AGENT_DEPTH;

/// Depth a new child of `parent` would occupy (`None` parent → depth 0).
pub fn child_depth(agents: &HashMap<String, SpawnedAgent>, parent: Option<&str>) -> Option<u8> {
    match parent {
        None => Some(0),
        Some(p) => agents.get(p).map(|a| a.depth + 1),
    }
}

/// Validate a spawn under `parent`, returning the child depth or an error
/// message (unknown parent or depth cap exceeded).
pub fn validate_spawn(
    agents: &HashMap<String, SpawnedAgent>,
    parent: Option<&str>,
) -> Result<u8, String> {
    match child_depth(agents, parent) {
        None => Err(format!(
            "Parent agent '{}' not found.",
            parent.unwrap_or("")
        )),
        Some(d) if d > MAX_AGENT_DEPTH => Err(format!(
            "Spawn denied: max agent depth {MAX_AGENT_DEPTH} reached."
        )),
        Some(d) => Ok(d),
    }
}
