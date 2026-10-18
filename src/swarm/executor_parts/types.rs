//! Swarm executor bound to the application stage adapters.
/// Application swarm execution with native agent and stage services.
pub type SwarmExecutor = codetether_swarm_runtime::SwarmExecutor<super::application_host::Native>;
