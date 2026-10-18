//! Native agent entry type for shared display ordering.
use super::agent_profile::SpawnedAgent;
pub use codetether_agent_tree::{AgentNode, MAX_AGENT_DEPTH};
codetether_agent_tree::define_display_order!(SpawnedAgent);
