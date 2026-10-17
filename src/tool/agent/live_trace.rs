//! Native session-event mapping for shared in-flight transcript snapshots.
#[path = "live_trace_event.rs"]
mod event;
pub(crate) use codetether_agent_trace::{LiveTraceEntry, LiveTraceSnapshot};
pub(in crate::tool::agent) use codetether_agent_trace::{begin, clear, snapshot};
pub(in crate::tool::agent) fn observe(name: &str, event: &crate::session::SessionEvent) {
    codetether_agent_trace::observe(name, self::event::project(event));
}
