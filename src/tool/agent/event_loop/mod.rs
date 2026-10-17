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
#[cfg(test)]
mod collect;
mod host;
#[path = "../live_trace.rs"]
pub(super) mod live_trace;
#[cfg(test)]
#[path = "../live_trace_flow_tests.rs"]
mod live_trace_flow_tests;
#[cfg(test)]
#[path = "../live_trace_tests.rs"]
mod live_trace_tests;
#[cfg(test)]
mod tests;

use crate::session::{Session, SessionEvent};
use serde_json::Value;
use tokio::sync::mpsc;

pub(super) type ChildTask = codetether_child_events::ChildTask<Session>;
/// Collect native events through the shared child-output collector.
pub(super) async fn run(
    agent_name: &str,
    rx: &mut mpsc::Receiver<SessionEvent>,
    handle: ChildTask,
) -> (String, String, Vec<Value>, Option<String>, Option<Session>) {
    codetether_child_events::run::<host::ApplicationHost, Session>(agent_name, rx, handle).await
}
