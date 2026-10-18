//! Native provider binding for shared swarm orchestration.
#[path = "orchestrator_host.rs"]
mod host;
/// Orchestrator using the application's Vault-backed provider registry.
pub type Orchestrator = codetether_swarm_orchestrator::Orchestrator<host::Native>;
pub use codetether_swarm_orchestrator::{OrchestratorMessage, SubAgentMessage};

pub(crate) use host::Native as NativeProvider;
