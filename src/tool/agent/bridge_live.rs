//! Parent-scoped access to an agent's in-flight TUI trace.

use super::store;

pub(crate) use super::super::event_loop::live_trace::{LiveTraceEntry, LiveTraceSnapshot};

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
pub(crate) fn agent_tool_live_trace_for_parent(
    name: &str,
    parent_session_id: &str,
) -> Option<LiveTraceSnapshot> {
    let entry = store::get_for_parent(name, Some(parent_session_id))?;
    super::super::event_loop::live_trace::snapshot(entry.id())
}
