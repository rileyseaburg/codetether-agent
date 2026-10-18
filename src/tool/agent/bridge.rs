//! Native session bindings for shared read-only agent views.
#[cfg(test)]
use super::store;
#[path = "bridge_host.rs"]
mod host;
pub use codetether_agent_view::{AgentOrigin, AgentSnapshot};
pub(super) use host::ApplicationHost;
#[path = "bridge_live.rs"]
mod live;
#[cfg(test)]
#[path = "bridge_test_fixture.rs"]
mod test_fixture;
#[path = "bridge_transcript.rs"]
mod transcript;
pub(crate) use live::{LiveTraceEntry, LiveTraceSnapshot, agent_tool_live_trace_for_parent};
#[cfg(test)]
pub(crate) use test_fixture::record_remote_turn;
pub(crate) use transcript::agent_tool_transcript_for_parent;

/// Returns snapshots of all agents spawned via the `agent` tool.
pub fn list_agent_tool_agents() -> Vec<AgentSnapshot> {
    codetether_agent_view::list::<ApplicationHost>(None)
}

/// Returns only agents owned by `parent_session_id`.
pub fn list_agent_tool_agents_for_parent(parent_session_id: &str) -> Vec<AgentSnapshot> {
    codetether_agent_view::list::<ApplicationHost>(Some(parent_session_id))
}

/// Looks up an agent only when it belongs to `parent_session_id`.
pub fn find_agent_tool_agent_for_parent(
    name: &str,
    parent_session_id: &str,
) -> Option<AgentSnapshot> {
    codetether_agent_view::find::<ApplicationHost>(name, parent_session_id)
}

#[cfg(test)]
#[path = "bridge_remote_tests.rs"]
mod remote_tests;
#[cfg(test)]
#[path = "bridge_tests.rs"]
mod tests;
