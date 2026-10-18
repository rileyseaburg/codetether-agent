//! Shared owner-scoped live-trace lookup through native registry access.
pub(crate) use codetether_agent_trace::{LiveTraceEntry, LiveTraceSnapshot};
pub(crate) fn agent_tool_live_trace_for_parent(
    name: &str,
    parent_session_id: &str,
) -> Option<LiveTraceSnapshot> {
    codetether_agent_view::live_trace::<super::ApplicationHost>(name, parent_session_id)
}
