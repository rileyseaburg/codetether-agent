//! Shared belief lifecycle and project memory services.
//!
//! [beliefs] owns structured claims, confidence updates and extraction decoding.
//! [palace] persists those beliefs under a project root and renders active context.
//! Model invocation remains a responsibility of the application.
pub mod beliefs;
pub mod palace;

/// Persona state and lifecycle payloads.
pub mod persona;
/// Proposal and voting configuration contracts.
pub mod proposal;
/// Thought event and per-tick data contracts.
pub mod thought;
/// Global workspace and attention records.
pub mod workspace;

pub use persona::{
    CreatePersonaRequest, PersonaIdentity, PersonaPolicy, PersonaRuntimeState, PersonaStatus,
    ReapPersonaRequest, SpawnPersonaRequest,
};
pub use proposal::{Proposal, ProposalRisk, ProposalStatus, ProposalVote, SwarmGovernance};
pub use thought::{
    MemorySnapshot, ThoughtEvent, ThoughtEventType, ThoughtPhase, ThoughtResult, ThoughtWorkItem,
};
pub use workspace::{AttentionItem, AttentionSource, GlobalWorkspace};

/// Storage-safe text trimming and simple fact-count heuristics.
pub mod text;

/// Versioned cognition-state capture and persistence.
pub mod persistence;

/// Start and stop control payloads.
pub mod control;
/// Runtime status and lineage response contracts.
pub mod status;

/// Execution records, leases, extraction policy and outcome events.
pub mod execution;

/// Runtime construction options and environment readers.
pub mod configuration;
/// Backend-neutral thinker contracts and configuration.
pub mod thinker;
pub use configuration::CognitionRuntimeOptions;
pub use thinker::{CandleDevicePreference, ThinkerBackend, ThinkerConfig, ThinkerOutput};

/// Cadence driving independent of host tick implementation.
pub mod driver;

/// FunctionGemma prompt and tool-call interpretation policy.
pub mod tool_routing;

/// Shared cognition state and loop-handle ownership.
pub mod runtime;

mod recall_binding;
