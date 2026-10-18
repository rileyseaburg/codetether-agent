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

codetether_child_events::declare_application_collector!(
    crate::session::Session,
    crate::session::SessionEvent,
    crate::approval::LiveApprovalRequest,
    crate::tool::agent::event_loop::live_trace::observe,
    crate::approval::approve_autonomous_child,
);
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
