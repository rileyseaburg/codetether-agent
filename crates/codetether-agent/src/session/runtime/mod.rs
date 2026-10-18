//! Shared application boundary for session execution control.
//!
//! [`steering`] adapts trusted input, IPC, and transcript delivery to the
//! independent runtime crate. Goal adapters own journal lookup, accounting,
//! answer holds, and automatic continuation. Frontends translate requests;
//! provider and tool loops consume the same session controls.
//!
//! # Examples
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! let accepted = codetether_agent::session::runtime::steering::send(
//!     "session-1234", "Use the cloned repositories",
//! ).await.unwrap();
//! assert!(!accepted); // No active owner for this example session.
//! # });
//! ```

pub(crate) mod goals;
pub mod steering;

/// One process-wide owner for frontend reservations and session executors.
pub(crate) use codetether_session_control::CONTROL;

codetether_session_orchestration::impl_session_transcript!(crate::session::Session);
pub(crate) use codetether_native_session_runtime::{
    RunGuard, boundary, continuation, execution_scope, swarm_monitor,
};
use codetether_session_control as handle;
pub use handle::ControlHandle;
#[cfg(test)]
mod continuation_handoff_tests;
#[cfg(test)]
mod execution_scope_tests;
#[cfg(test)]
mod handle_spawn_tests;
#[cfg(test)]
mod swarm_monitor_tests;
