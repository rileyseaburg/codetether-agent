//! Perpetual cognition runtime for persona swarms.
//!
//! Personas think on a fixed cadence, rotating through observe, reflect, test,
//! and compress phases. Thoughts become beliefs, beliefs become proposals, and
//! proposals execute only after governance gates pass.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::cognition::{CognitionRuntime, CognitionRuntimeOptions};
//!
//! let runtime = CognitionRuntime::new_with_options(CognitionRuntimeOptions::default());
//! assert!(!runtime.is_enabled());
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
pub mod beliefs;
pub mod executor;
pub mod persistence;
mod thinker;

// The real router needs the in-process Candle runtime, so it requires both
// `functiongemma` and `candle`. Every other build gets the no-op stand-in.
#[cfg(all(feature = "functiongemma", feature = "candle"))]
pub mod tool_router;
#[cfg(not(all(feature = "functiongemma", feature = "candle")))]
#[allow(dead_code)]
#[path = "tool_router_disabled.rs"]
pub mod tool_router;

// ── Contracts ───────────────────────────────────────────────────────────
#[path = "attention.rs"]
mod attention;
#[path = "control_request.rs"]
mod control_request;
#[path = "governance.rs"]
mod governance;
#[path = "persona.rs"]
mod persona;
#[path = "persona_policy.rs"]
mod persona_policy;
#[path = "persona_request.rs"]
mod persona_request;
#[path = "persona_state.rs"]
mod persona_state;
#[path = "proposal.rs"]
mod proposal;
#[path = "proposal_kind.rs"]
mod proposal_kind;
#[path = "runtime_options.rs"]
mod runtime_options;
#[path = "runtime_state.rs"]
mod runtime_state;
#[path = "status.rs"]
mod status;
#[path = "thought_event.rs"]
mod thought_event;
#[path = "thought_event_type.rs"]
mod thought_event_type;
#[path = "thought_phase.rs"]
mod thought_phase;
#[path = "thought_work.rs"]
mod thought_work;
#[path = "workspace.rs"]
mod workspace;

// ── Runtime API ─────────────────────────────────────────────────────────
#[path = "persona_build.rs"]
mod persona_build;
#[path = "persona_create.rs"]
mod persona_create;
#[path = "persona_depth.rs"]
mod persona_depth;
#[path = "persona_lineage.rs"]
mod persona_lineage;
#[path = "persona_reap.rs"]
mod persona_reap;
#[path = "persona_reap_targets.rs"]
mod persona_reap_targets;
#[path = "persona_spawn.rs"]
mod persona_spawn;
#[path = "runtime_approval.rs"]
mod runtime_approval;
#[path = "runtime_assemble.rs"]
mod runtime_assemble;
#[path = "runtime_lineage.rs"]
mod runtime_lineage;
#[path = "runtime_new.rs"]
mod runtime_new;
#[path = "runtime_query.rs"]
mod runtime_query;
#[path = "runtime_start.rs"]
mod runtime_start;
#[path = "runtime_status.rs"]
mod runtime_status;
#[path = "runtime_stop.rs"]
mod runtime_stop;

// ── Construction ────────────────────────────────────────────────────────
#[path = "env.rs"]
mod env;
#[path = "initial_state.rs"]
mod initial_state;
#[path = "options_env.rs"]
mod options_env;
#[path = "thinker_env.rs"]
mod thinker_env;
#[path = "thinker_env_candle.rs"]
mod thinker_env_candle;
#[path = "thinker_init.rs"]
mod thinker_init;

// ── Cognition loop ──────────────────────────────────────────────────────
#[path = "buffers.rs"]
mod buffers;
#[path = "idle_detect.rs"]
mod idle_detect;
#[path = "loop_ctx.rs"]
mod loop_ctx;
#[path = "loop_drive.rs"]
mod loop_drive;
#[path = "tick_budget.rs"]
mod tick_budget;
#[path = "tick_charge.rs"]
mod tick_charge;
#[path = "tick_compress.rs"]
mod tick_compress;
#[path = "tick_event.rs"]
mod tick_event;
#[path = "tick_flush.rs"]
mod tick_flush;
#[path = "tick_govern.rs"]
mod tick_govern;
#[path = "tick_idle_reap.rs"]
mod tick_idle_reap;
#[path = "tick_process.rs"]
mod tick_process;
#[path = "tick_progress.rs"]
mod tick_progress;
#[path = "tick_propose.rs"]
mod tick_propose;
#[path = "tick_reflect.rs"]
mod tick_reflect;
#[path = "tick_run.rs"]
mod tick_run;
#[path = "tick_snapshot.rs"]
mod tick_snapshot;
#[path = "tick_snapshot_meta.rs"]
mod tick_snapshot_meta;
#[path = "tick_test.rs"]
mod tick_test;
#[path = "tick_test_phase.rs"]
mod tick_test_phase;
#[path = "tick_window.rs"]
mod tick_window;

// ── Beliefs, proposals, governance ──────────────────────────────────────
#[path = "belief_confirm.rs"]
mod belief_confirm;
#[path = "belief_contest.rs"]
mod belief_contest;
#[path = "belief_decay.rs"]
mod belief_decay;
#[path = "belief_merge.rs"]
mod belief_merge;
#[path = "proposal_create.rs"]
mod proposal_create;
#[path = "proposal_event.rs"]
mod proposal_event;
#[path = "proposal_execute.rs"]
mod proposal_execute;
#[path = "proposal_receipt.rs"]
mod proposal_receipt;
#[path = "vote_attention.rs"]
mod vote_attention;
#[path = "vote_deadline.rs"]
mod vote_deadline;
#[path = "vote_resolve.rs"]
mod vote_resolve;
#[path = "vote_sweep.rs"]
mod vote_sweep;
#[path = "vote_tally.rs"]
mod vote_tally;
#[path = "workspace_attention.rs"]
mod workspace_attention;
#[path = "workspace_rank.rs"]
mod workspace_rank;
#[path = "workspace_refresh.rs"]
mod workspace_refresh;
#[path = "workspace_uncertain.rs"]
mod workspace_uncertain;

// ── Thought text ────────────────────────────────────────────────────────
#[path = "context_select.rs"]
mod context_select;
#[path = "context_signals.rs"]
mod context_signals;
#[path = "context_summary.rs"]
mod context_summary;
#[path = "defaults.rs"]
mod defaults;
#[path = "normalize.rs"]
mod normalize;
#[path = "normalize_shape.rs"]
mod normalize_shape;
#[path = "phase_default.rs"]
mod phase_default;
#[path = "placeholder.rs"]
mod placeholder;
#[path = "prompt_build.rs"]
mod prompt_build;
#[path = "prompt_phase.rs"]
mod prompt_phase;
#[path = "text_util.rs"]
pub mod text_util;
#[path = "thought_generate.rs"]
mod thought_generate;
#[path = "thought_result.rs"]
mod thought_result;

// ── Tests ───────────────────────────────────────────────────────────────
#[cfg(test)]
#[path = "budget_tests.rs"]
mod budget_tests;
#[cfg(test)]
#[path = "governance_quorum_tests.rs"]
mod governance_quorum_tests;
#[cfg(test)]
#[path = "governance_tests_support.rs"]
mod governance_tests_support;
#[cfg(test)]
#[path = "governance_veto_tests.rs"]
mod governance_veto_tests;
#[cfg(test)]
#[path = "idle_reap_tests.rs"]
mod idle_reap_tests;
#[cfg(test)]
#[path = "lifecycle_tests.rs"]
mod lifecycle_tests;
#[cfg(test)]
#[path = "normalize_tests.rs"]
mod normalize_tests;
#[cfg(test)]
#[path = "persona_branch_tests.rs"]
mod persona_branch_tests;
#[cfg(test)]
#[path = "persona_depth_tests.rs"]
mod persona_depth_tests;
#[cfg(test)]
#[path = "persona_tests.rs"]
mod persona_tests;
#[cfg(test)]
#[path = "tests_request.rs"]
mod tests_request;
#[cfg(test)]
#[path = "tests_support.rs"]
mod tests_support;
#[cfg(test)]
#[path = "tick_budget_tests.rs"]
mod tick_budget_tests;
#[cfg(test)]
#[path = "tick_budget_tests_support.rs"]
mod tick_budget_tests_support;
#[cfg(test)]
#[path = "workspace_tests.rs"]
mod workspace_tests;

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
