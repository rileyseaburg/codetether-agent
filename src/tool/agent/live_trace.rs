//! Native session-event mapping for shared in-flight transcript snapshots.
pub(in crate::tool::agent) use codetether_agent_trace::observe_session as observe;
pub(crate) use codetether_agent_trace::{LiveTraceEntry, LiveTraceSnapshot};
pub(in crate::tool::agent) use codetether_agent_trace::{begin, clear, snapshot};
