//! Application provider setup and adapters for shared swarm planning/state.
//!
//! [Orchestrator] resolves a provider, generates plans through the shared planner,
//! and exposes the shared coordinator state through the existing execution API.
use super::{DecompositionStrategy, SubAgent, SubTask, SubTaskResult, SwarmConfig, SwarmStats};
use crate::provider::ProviderRegistry;
use anyhow::Result;
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
pub(crate) use model_policy::{choose_default_provider, default_model_for_provider};

/// The orchestrator manages task decomposition and sub-agent coordination
pub struct Orchestrator {
    state: codetether_swarm_core::coordinator::CoordinatorState,
    /// Configuration
    config: SwarmConfig,

    /// Provider registry for AI calls
    providers: ProviderRegistry,

    /// Current model for orchestration
    model: String,

    /// Current provider
    provider: String,
}
