//! `status` action: report each sub-agent's latest state and liveness.
//!
//! Joins the spawned-agent registry ([`store`]) with the newest bus
//! [`TaskState`](crate::a2a::types::TaskState) so the parent agent can see
//! whether each sub-agent is active, stalled, or settled — the visibility the
//! synchronous tool-return channel never provided.

use serde_json::{Value, json};

use super::bus_publish::lifecycle;
use super::status_liveness::liveness;
use super::status_source::{AgentStatus, latest_states};
use super::store;
use crate::tool::ToolResult;

/// Build the JSON status report for sub-agents owned by the calling session.
pub(super) fn handle_status(parent: Option<&str>) -> ToolResult {
    let agents = store::entries_for_parent(parent);
    if agents.is_empty() {
        return ToolResult::success("No sub-agents spawned. Use action \"spawn\".");
    }
    let states = latest_states(&lifecycle::task_to_agent(parent));
    let rows: Vec<Value> = agents
        .iter()
        .map(|entry| {
            row(
                &entry.name,
                entry.id(),
                entry.session.messages.len(),
                states.get(entry.id()),
            )
        })
        .collect();
    ToolResult::success(serde_json::to_string_pretty(&rows).unwrap_or_default())
}

fn row(name: &str, session_id: &str, messages: usize, status: Option<&AgentStatus>) -> Value {
    match status {
        Some(st) => json!({
            "agent": name,
            "agent_id": session_id,
            "messages": messages,
            "state": format!("{:?}", st.state),
            "liveness": liveness(&st.state, st.at),
            "last_update": st.at.to_rfc3339(),
            "summary": st.summary,
        }),
        None => json!({
            "agent": name,
            "agent_id": session_id,
            "messages": messages,
            "state": "Unknown",
            "liveness": "no_activity",
            "last_update": Value::Null,
            "summary": Value::Null,
        }),
    }
}
