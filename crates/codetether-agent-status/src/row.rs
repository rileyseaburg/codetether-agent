//! Format one live child and its optional latest status.
use crate::status_liveness::liveness;
use crate::status_source::AgentStatus;
use serde_json::{Value, json};

pub(crate) fn row(
    name: &str,
    session_id: &str,
    messages: usize,
    status: Option<&AgentStatus>,
) -> Value {
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
