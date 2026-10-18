//! Parent-scoped access to an agent's in-flight TUI trace.

use crate::Host;

use codetether_agent_trace::LiveTraceSnapshot;

/// Return live trace data only when the agent belongs to the parent session.
///
/// # Arguments
///
/// * `name` - Child agent name.
/// * `parent_session_id` - Session that must own the child.
///
/// # Returns
///
/// The active trace, or `None` for an idle, missing, or foreign child.
///
/// # Examples
///
/// ```ignore
/// let trace = agent_tool_live_trace_for_parent("reviewer", "parent-id");
/// ```
pub fn for_parent<H: Host>(name: &str, parent_session_id: &str) -> Option<LiveTraceSnapshot> {
    H::get_for_parent(name, parent_session_id)
        .and_then(|entry| codetether_agent_trace::snapshot(entry.id()))
        .or_else(|| codetether_peer_agent::observation::live_trace(name, parent_session_id))
}
