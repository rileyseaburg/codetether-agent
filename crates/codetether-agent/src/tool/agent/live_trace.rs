//! Native session-event mapping for shared in-flight transcript snapshots.
#[cfg(not(test))]
pub(in crate::tool::agent) use codetether_agent_trace::clear;
pub(in crate::tool::agent) use codetether_agent_trace::observe_session as observe;

#[cfg(test)]
include!("live_trace_test_imports.rs");
