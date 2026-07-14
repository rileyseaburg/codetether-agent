//! Public bridge for TUI observability of agent-tool-spawned sub-agents.
//!
//! The TUI has its own `spawned_agents` map for `/spawn`-created agents, but
//! agents spawned via the `agent` tool live in the private [`store`]. This
//! module exposes a read-only snapshot so the TUI's `/agents`, `/bus`, and
//! `/swarm` views can display all live sub-agents regardless of origin
//! (issue #295 / #297 Part A).

use super::store;
#[path = "bridge_transcript.rs"]
mod transcript;
pub(crate) use transcript::agent_tool_transcript_for_parent;

/// A read-only snapshot of a spawned sub-agent, suitable for TUI display.
#[derive(Clone, Debug)]
pub struct AgentSnapshot {
    pub name: String,
    pub instructions: String,
    pub message_count: usize,
    pub model_id: Option<String>,
    pub parent: Option<String>,
    pub depth: u8,
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
        .find(|agent| agent.name == name)
}

fn snapshots(parent_session_id: Option<&str>) -> Vec<AgentSnapshot> {
    let rows = store::list_with_metadata(parent_session_id);
    rows.into_iter()
        .map(
            |(name, instructions, msg_count, model_id, parent, depth)| AgentSnapshot {
                name,
                instructions,
                message_count: msg_count,
                model_id,
                parent,
                depth,
            },
        )
        .collect()
}

#[cfg(test)]
#[path = "bridge_tests.rs"]
mod tests;
