//! Native worker startup state, registration and workspace synchronization.
//!
//! [register_worker] submits the shared A2A payload using cached native provider
//! discovery. [bridge::TuiWorkerBridge] uses that same registration function
//! when the terminal starts a worker.
mod registration;
pub use registration::{register_current_worker, register_worker};
pub mod bridge;

/// Shared default models and temperature policy for worker execution.
pub mod model_policy;

/// Shared task runtime using native HTTP and bus types.
pub mod runtime_state;
/// Concrete startup context using shared command, bus and peer types.
pub mod worker_context;
/// Workspace lookup and task-progress synchronization.
pub mod workspace_scope;
/// Repository-authenticated workspace synchronization.
pub mod workspace_sync;
/// Periodic workspace synchronization startup.
pub mod workspace_sync_start;
use codetether_a2a_worker_core::task_timeline;
use codetether_worktree::repository::credentials as workspace_sync_git;
pub use runtime_state::WorkerTaskRuntime;
pub use worker_context::WorkerContext;

/// Worker initialization with application-specific catalog and peer hooks.
pub mod initialization;

/// Registration, initial synchronization and pending-task bootstrap sequence.
pub mod bootstrap;
/// Pending-task transport with application-supplied claim dispatch.
pub mod pending;

/// Standalone and HTTP-integrated reconnect loops.
pub mod reconnect;
/// Resumable worker stream consumption using the native task dispatcher hook.
pub mod stream;

/// Standalone and HTTP-integrated worker startup orchestration.
pub mod entrypoints;

/// Task admission, claim, progress and release with application execution hooks.
pub mod lifecycle;

/// Structured event-sink installation and cleanup for worker tasks.
pub mod event_sink;
/// Policy-gated clone and forage dispatch with application forage hooks.
pub mod special;

/// Claimed-task composition using application session and forage hooks.
pub mod claimed;

/// Worker session preparation with application persistence and metadata access.
pub mod preparation;

/// Context-aware worker completion, compact-schema fallback and goal accounting.
pub mod completion;

/// Scoped worker session steps with application response execution.
pub mod session_loop;

/// Response sequencing with native telemetry and application tool-call effects.
pub mod response;
/// Native registry lookup and policy-gated tool execution.
pub mod tools;

mod session_steps;
/// Worker tool-call approval, enrichment, execution and result recording.
pub mod tool_call;
pub use session_steps::run_session_steps;

/// Standard worker policy using application session bookkeeping and tool assembly.
pub mod session_policy;

/// Native registry/workspace tool construction with application workflows.
pub mod tool_registry;

/// Controlled swarm execution, result persistence and standard-session handoff.
pub mod swarm_policy;
