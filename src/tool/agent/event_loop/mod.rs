//! Event collection for sub-agent message runs.
//!
//! This module coordinates streaming session events and final handle
//! resolution for `message` actions.
//!
//! # Examples
//!
//! ```ignore
//! let result = run("reviewer", &mut rx, handle).await;
//! ```

mod approve;
mod collect;
mod finalize;
mod handle;
#[path = "../live_trace.rs"]
pub(super) mod live_trace;
#[cfg(test)]
#[path = "../live_trace_flow_tests.rs"]
mod live_trace_flow_tests;
#[cfg(test)]
#[path = "../live_trace_tests.rs"]
mod live_trace_tests;
mod state;
mod task;
#[cfg(test)]
mod tests;

use crate::session::{Session, SessionEvent};
use serde_json::Value;
use tokio::sync::mpsc;

pub(super) use task::ChildTask;

/// Collects streaming output and resolves the spawned task handle.
///
/// # Examples
///
/// ```ignore
/// let (response, thinking, tools, error, session) = run("reviewer", &mut rx, handle).await;
/// ```
pub(super) async fn run(
    agent_name: &str,
    rx: &mut mpsc::Receiver<SessionEvent>,
    handle: ChildTask,
) -> (String, String, Vec<Value>, Option<String>, Option<Session>) {
    let mut state = collect::collect_events(agent_name, rx).await;
    let updated_session = finalize::finish_handle(handle, state.timed_out, &mut state.error).await;
    (
        state.response,
        state.thinking,
        state.tools,
        state.error,
        updated_session,
    )
}
