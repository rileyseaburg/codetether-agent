//! Cognition runtime facade with application-selected services.
//!
//! [CognitionRuntime] owns shared state and exposes lifecycle, persona and
//! query operations. [Host] supplies initialization, the thinker, executable
//! tool registry and background loop driver. Core state and mutation policy
//! remain in codetether-cognition-core.
use codetether_cognition_core::control::StartCognitionRequest;
use codetether_cognition_core::runtime::InitialState;
use codetether_cognition_core::status::{CognitionStatus, LineageGraph, ReapPersonaResponse};
use codetether_cognition_core::{
    AttentionItem, CognitionRuntimeOptions, CreatePersonaRequest, GlobalWorkspace, MemorySnapshot,
    PersonaRuntimeState, Proposal, ReapPersonaRequest, SpawnPersonaRequest, SwarmGovernance,
    ThinkerConfig, ThoughtEvent,
};
mod host;
mod persona_create;
mod persona_reap;
mod persona_spawn;
mod runtime_approval;
mod runtime_assemble;
mod runtime_lineage;
mod runtime_new;
mod runtime_query;
mod runtime_start;
mod runtime_state;
mod runtime_status;
mod runtime_stop;
pub use host::Host;
pub use runtime_state::CognitionRuntime;

mod drive;
pub use drive::{drive, run_tick};

/// Standard thinker, storage and tick composition for application runtimes.
pub mod application;
mod http;
#[cfg(test)]
mod tests;
