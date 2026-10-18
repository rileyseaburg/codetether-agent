//! Perpetual cognition runtime for persona swarms.
//!
//! Personas think on a fixed cadence, rotating through observe, reflect, test,
//! and compress phases. Thoughts become beliefs, beliefs become proposals, and
//! proposals execute only after governance gates pass.
//!
//! # Examples
//!
//! ```rust
//! use codetether_native_cognition::{CognitionRuntime, CognitionRuntimeOptions};
//! # fn example<R: codetether_cognition_core::execution::dispatch::Registry + std::fmt::Debug + 'static>() {
//!
//! let runtime = CognitionRuntime::<R, false>::new_with_options(CognitionRuntimeOptions::default());
//! assert!(!runtime.is_enabled());
//! # }
//! ```
//!
//! # Architecture
//!
//! Contracts live in `persona*`, `proposal*`, `thought_event*`, `attention`,
//! `governance`, `workspace`, and `status`. Runtime behavior is split into
//! `runtime_*` (public API) and `tick_*` / `loop_*` (the cognition loop).
//! Thought text handling lives in `prompt_*`, `normalize`, `phase_default`, and
//! `text_util`.

// ── Subsystems ──────────────────────────────────────────────────────────
include!("subsystem_modules.rs");

pub use codetether_native_thinker::tool_router;

// ── Contracts ───────────────────────────────────────────────────────────
use codetether_cognition_core::control as control_request;
use codetether_cognition_core::persona;
use codetether_cognition_core::persona as persona_policy;
use codetether_cognition_core::persona as persona_request;
use codetether_cognition_core::persona as persona_state;
use codetether_cognition_core::proposal as governance;
use codetether_cognition_core::proposal;
use codetether_cognition_core::proposal as proposal_kind;
use codetether_cognition_core::workspace as attention;
mod runtime_host;
use codetether_cognition_core::configuration as runtime_options;
#[path = "runtime_state.rs"]
mod runtime_state;
use codetether_cognition_core::status;
use codetether_cognition_core::thought as thought_event;
use codetether_cognition_core::thought as thought_event_type;
use codetether_cognition_core::workspace;

// ── Runtime API ─────────────────────────────────────────────────────────

// ── Construction ────────────────────────────────────────────────────────

// ── Cognition loop ──────────────────────────────────────────────────────

// ── Beliefs, proposals, governance ──────────────────────────────────────

// ── Thought text ────────────────────────────────────────────────────────
pub use codetether_native_thinker::text_util;

#[cfg(test)]
mod tests;

// ── Public surface ──────────────────────────────────────────────────────
pub use attention::{AttentionItem, AttentionSource};
pub use control_request::{StartCognitionRequest, StopCognitionRequest};
pub use governance::SwarmGovernance;
pub use persona::{PersonaIdentity, PersonaStatus};
pub use persona_policy::PersonaPolicy;
pub use persona_request::{CreatePersonaRequest, ReapPersonaRequest, SpawnPersonaRequest};
pub use persona_state::PersonaRuntimeState;
pub use proposal::Proposal;
pub use proposal_kind::{ProposalRisk, ProposalStatus, ProposalVote};
pub use runtime_options::CognitionRuntimeOptions;
pub use runtime_state::CognitionRuntime;
pub use status::{CognitionStatus, LineageGraph, LineageNode, ReapPersonaResponse};
pub use thinker::{
    CandleDevicePreference, ThinkerBackend, ThinkerClient, ThinkerConfig, ThinkerOutput,
};
pub use thought_event::{MemorySnapshot, ThoughtEvent};
pub use thought_event_type::ThoughtEventType;
pub use workspace::GlobalWorkspace;
