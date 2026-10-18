//! Native provider binding for shared swarm orchestration.
/// Orchestrator using the application's Vault-backed provider registry.
pub type Orchestrator =
    codetether_swarm_orchestrator::Orchestrator<codetether_native_swarm::Providers>;
pub use codetether_swarm_orchestrator::{OrchestratorMessage, SubAgentMessage};
