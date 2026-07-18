//! Public bridge for TUI observability of agent-tool-spawned sub-agents.
//!
//! The TUI has its own `spawned_agents` map for `/spawn`-created agents, but
//! agents spawned via the `agent` tool live in the private [`store`]. This
//! module exposes a read-only snapshot so the TUI's `/agents`, `/bus`, and
//! `/swarm` views can display all live sub-agents regardless of origin
//! (issue #295 / #297 Part A).

use super::store;
#[path = "bridge_live.rs"]
mod live;
#[path = "bridge_transcript.rs"]
mod transcript;
pub(crate) use live::{LiveTraceEntry, LiveTraceSnapshot, agent_tool_live_trace_for_parent};
pub(crate) use transcript::agent_tool_transcript_for_parent;

/// A read-only snapshot of a spawned sub-agent, suitable for TUI display.
#[derive(Clone, Debug)]
pub struct AgentSnapshot {
    pub id: String,
    pub name: String,
    pub instructions: String,
    pub message_count: usize,
    pub model_id: Option<String>,
    pub parent: Option<String>,
    pub depth: u8,
    pub is_processing: bool,
}

/// Returns snapshots of all agents spawned via the `agent` tool.
pub fn list_agent_tool_agents() -> Vec<AgentSnapshot> {
    snapshots(None)
}

/// Returns only agents owned by `parent_session_id`.
pub fn list_agent_tool_agents_for_parent(parent_session_id: &str) -> Vec<AgentSnapshot> {
    snapshots(Some(parent_session_id))
}

/// Looks up an agent only when it belongs to `parent_session_id`.
pub fn find_agent_tool_agent_for_parent(
    name: &str,
    parent_session_id: &str,
) -> Option<AgentSnapshot> {
    snapshots(Some(parent_session_id))
        .into_iter()
        .find(|agent| agent.name == name || agent.id == name)
}

fn snapshots(parent_session_id: Option<&str>) -> Vec<AgentSnapshot> {
    store::entries_for_parent(parent_session_id)
        .into_iter()
        .map(|entry| AgentSnapshot {
            id: entry.session.id.clone(),
            name: entry.name,
            instructions: entry.instructions,
            message_count: entry.session.messages.len(),
            model_id: entry.model_id,
            parent: entry.parent,
            depth: entry.depth,
            is_processing: super::execution_state::is_running(&entry.session.id),
        })
        .collect()
}

#[cfg(test)]
#[path = "bridge_tests.rs"]
mod tests;
