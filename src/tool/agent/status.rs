//! `status` action: report each sub-agent's latest state and liveness.
//!
//! Joins the spawned-agent registry ([`store`]) with the newest bus
//! [`TaskState`](crate::a2a::types::TaskState) so the parent agent can see
//! whether each sub-agent is active, stalled, or settled — the visibility the
//! synchronous tool-return channel never provided.

use serde_json::{Value, json};

use super::status_liveness::liveness;
use super::status_source::{AgentStatus, latest_states};
use super::store;
use crate::tool::ToolResult;

/// Build the JSON status report for all known sub-agents.
pub(super) fn handle_status() -> ToolResult {
    let states = latest_states();
    let agents = store::list();
    if agents.is_empty() && states.is_empty() {
        return ToolResult::success("No sub-agents spawned. Use action \"spawn\".");
    }
    let mut rows: Vec<Value> = agents
        .iter()
        .map(|(name, _, msgs)| row(name, *msgs, states.get(name)))
        .collect();
    for (name, st) in &states {
        if !agents.iter().any(|(n, _, _)| n == name) {
            rows.push(row(name, 0, Some(st)));
        }
    }
    ToolResult::success(serde_json::to_string_pretty(&rows).unwrap_or_default())
}

fn row(name: &str, messages: usize, status: Option<&AgentStatus>) -> Value {
    match status {
        Some(st) => json!({
            "agent": name,
            "messages": messages,
            "state": format!("{:?}", st.state),
            "liveness": liveness(&st.state, st.at),
            "last_update": st.at.to_rfc3339(),
            "summary": st.summary,
        }),
        None => json!({
            "agent": name,
            "messages": messages,
            "state": "Unknown",
            "liveness": "no_activity",
            "last_update": Value::Null,
            "summary": Value::Null,
        }),
    }
}
