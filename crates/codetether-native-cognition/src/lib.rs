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
use codetether_cognition_core::thought as thought_phase;
use codetether_cognition_core::thought as thought_work;
use codetether_cognition_core::workspace;

// ── Runtime API ─────────────────────────────────────────────────────────
use codetether_cognition_core::persona::depth as persona_depth;
use codetether_cognition_core::persona::lifecycle as persona_build;
use codetether_cognition_core::persona::lineage as persona_lineage;
use codetether_cognition_core::persona::reap_targets as persona_reap_targets;

// ── Construction ────────────────────────────────────────────────────────
use codetether_cognition_core::configuration as options_env;
use codetether_cognition_core::configuration::env;
use codetether_cognition_core::thinker as thinker_env;
use codetether_cognition_core::thinker::candle_env as thinker_env_candle;

// ── Cognition loop ──────────────────────────────────────────────────────
use codetether_cognition_core::persona::accounting as tick_charge;
use codetether_cognition_core::persona::progress as tick_progress;
use codetether_cognition_core::persona::scheduling as idle_detect;
use codetether_cognition_core::persona::scheduling as tick_budget;
use codetether_cognition_core::persona::scheduling as tick_idle_reap;
use codetether_cognition_core::persona::scheduling as tick_window;
use codetether_cognition_core::thought::buffers;
use codetether_cognition_core::thought::build as tick_event;
use codetether_cognition_core::thought::build as tick_snapshot;
use codetether_cognition_core::thought::build as tick_snapshot_meta;

// ── Beliefs, proposals, governance ──────────────────────────────────────
use codetether_cognition_core::beliefs::update as belief_confirm;
use codetether_cognition_core::beliefs::update as belief_contest;
use codetether_cognition_core::beliefs::update as belief_decay;
use codetether_cognition_core::beliefs::update as belief_merge;
use codetether_cognition_core::proposal::build as proposal_create;
use codetether_cognition_core::proposal::build as proposal_event;
use codetether_cognition_core::proposal::dispatch as proposal_execute;
use codetether_cognition_core::proposal::dispatch as proposal_receipt;
use codetether_cognition_core::proposal::voting as vote_attention;
use codetether_cognition_core::proposal::voting as vote_deadline;
use codetether_cognition_core::proposal::voting as vote_resolve;
use codetether_cognition_core::proposal::voting as vote_sweep;
use codetether_cognition_core::proposal::voting as vote_tally;
use codetether_cognition_core::workspace::ranking as workspace_attention;
use codetether_cognition_core::workspace::ranking as workspace_rank;
use codetether_cognition_core::workspace::ranking as workspace_uncertain;
use codetether_cognition_core::workspace::refresh as workspace_refresh;

// ── Thought text ────────────────────────────────────────────────────────
use codetether_cognition_core::thought::context as context_select;
use codetether_cognition_core::thought::presentation as context_signals;
use codetether_cognition_core::thought::presentation as context_summary;
use codetether_cognition_core::thought::presentation as normalize;
use codetether_cognition_core::thought::presentation as normalize_shape;
use codetether_cognition_core::thought::presentation as phase_default;
use codetether_cognition_core::thought::presentation as placeholder;
use codetether_cognition_core::thought::presentation as prompt_build;
use codetether_cognition_core::thought::presentation as prompt_phase;
use codetether_cognition_core::thought::result as thought_result;
use codetether_native_thinker::defaults;
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

use thought_phase::ThoughtPhase;
use thought_work::{ThoughtResult, ThoughtWorkItem};
