//! Native session specialization of shared spawned-agent metadata.
pub use codetether_agent_identity::agent_profile;
/// Spawned-agent metadata carrying a native conversation session.
pub type SpawnedAgent = codetether_agent_tree::SpawnedAgent<crate::session::Session>;
