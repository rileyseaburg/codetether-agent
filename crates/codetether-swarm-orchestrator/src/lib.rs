//! Swarm model selection, decomposition requests and task coordination.
//!
//! Implement [Host] for provider-registry loading and lookup, then construct an
//! [Orchestrator]. It owns selection policy, generated plans and coordinator
//! state while exposing the same provider registry to native worker execution.
use anyhow::Result;
use codetether_swarm_core::{
    DecompositionStrategy, SubAgent, SubTask, SubTaskResult, SwarmConfig, SwarmStats,
};
mod host;
pub use host::Host;
mod access;
mod completion;
mod construct;
mod decompose;
mod decomposition_request;
mod lifecycle;
mod model_policy;
mod queries;
mod selection;
pub use codetether_swarm_core::{OrchestratorMessage, SubAgentMessage};
use model_policy::{choose_default_provider, default_model_for_provider};
/// Plan tasks and track sub-agent coordination with a native provider registry.
/// # Examples
/// ```no_run
/// async fn plan<H: codetether_swarm_orchestrator::Host>() -> anyhow::Result<()> {
///     let planner = codetether_swarm_orchestrator::Orchestrator::<H>::new(Default::default()).await?;
///     assert!(planner.all_subtasks().is_empty());
///     Ok(())
/// }
/// ```
pub struct Orchestrator<H: Host> {
    state: codetether_swarm_core::coordinator::CoordinatorState,
    config: SwarmConfig,
    providers: H::Providers,
    model: String,
    provider: String,
}
