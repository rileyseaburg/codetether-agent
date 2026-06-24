use super::{record::SubAgentRunRecord, state::SubAgentRunState};
use serde_json::Value;

/// Inspects a CodeTether child session JSON value and returns orphan-aware status.
///
/// A session with exactly the bootstrap message, zero tool uses, and no token usage
/// is `Orphaned` because the worker never advanced beyond creation.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::codetether_agent::subagent_orphans::classify_child_session_value;
/// use serde_json::json;
///
/// let value = json!({"id":"child","agent":"auditor","messages":[{"role":"system"}],"tool_uses":[],"usage":{"total_tokens":0}});
/// let record = classify_child_session_value(&value).unwrap();
/// assert_eq!(record.state.as_str(), "Orphaned");
/// ```
#[must_use]
pub fn classify_child_session_value(value: &Value) -> Option<SubAgentRunRecord> {
    let child_session_id = value.get("id")?.as_str()?.to_owned();
    let agent_name = agent_name(value)?.to_owned();
    let message_count = value
        .get("messages")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let tool_use_count = value
        .get("tool_uses")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let total_tokens = value
        .pointer("/usage/total_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let last_heartbeat_at = value
        .get("last_heartbeat_at")
        .and_then(Value::as_str)
        .map(str::to_owned);
    let state = classify_counts(
        message_count,
        tool_use_count,
        total_tokens,
        last_heartbeat_at.is_some(),
    );
    let reason = reason_for(state).to_owned();
    Some(SubAgentRunRecord {
        agent_name,
        child_session_id,
        state,
        message_count,
        tool_use_count,
        last_heartbeat_at,
        reason,
    })
}

fn agent_name(value: &Value) -> Option<&str> {
    value.get("agent").and_then(Value::as_str).or_else(|| {
        value
            .pointer("/metadata/provenance/identity/agent_name")
            .and_then(Value::as_str)
    })
}

fn classify_counts(
    messages: usize,
    tools: usize,
    tokens: u64,
    has_heartbeat: bool,
) -> SubAgentRunState {
    if messages <= 1 && tools == 0 && tokens == 0 && !has_heartbeat {
        SubAgentRunState::Orphaned
    } else if tools > 0 || tokens > 0 || has_heartbeat {
        SubAgentRunState::Running
    } else {
        SubAgentRunState::Created
    }
}

fn reason_for(state: SubAgentRunState) -> &'static str {
    match state {
        SubAgentRunState::Orphaned => {
            "child session was created but no worker heartbeat, tool use, token usage, or output was recorded"
        }
        SubAgentRunState::Running => "child session recorded worker activity",
        _ => "child session exists but has not reached a terminal state",
    }
}
