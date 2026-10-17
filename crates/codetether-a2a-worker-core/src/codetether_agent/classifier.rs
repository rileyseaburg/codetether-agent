//! Classify child-session records from their activity fields.

use super::record::SubAgentRunRecord;
use serde_json::Value;
mod activity;
mod name;
use activity::{classify_counts, reason_for};
use name::agent_name;

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
