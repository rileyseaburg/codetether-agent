//! Repair malformed tool-call/result sequences before sending upstream.
//!
//! OpenAI-compatible APIs (Cerebras included) reject a request where an
//! assistant message carrying `tool_calls` is not immediately answered by a
//! `tool` message for every `tool_call_id`. This happens when a user presses
//! "continue" while a tool call is still pending, inserting a user turn right
//! after a dangling tool call. Such a request faults with a 400 and surfaces
//! as `stream faulted (transient=false)`. We splice in synthetic tool results
//! so the conversation stays well-formed.

use serde_json::{Value, json};

const INTERRUPTED: &str = "[no result: tool call was interrupted]";

/// Insert synthetic `tool` messages for any unanswered assistant tool calls.
pub(super) fn repair_tool_results(messages: Vec<Value>) -> Vec<Value> {
    let mut out: Vec<Value> = Vec::with_capacity(messages.len());
    for (idx, msg) in messages.iter().enumerate() {
        let ids = pending_tool_ids(msg);
        out.push(msg.clone());
        if ids.is_empty() {
            continue;
        }
        let answered = answered_ids(&messages, idx + 1, &ids);
        for id in ids.iter().filter(|id| !answered.contains(*id)) {
            out.push(json!({
                "role": "tool",
                "tool_call_id": id,
                "content": INTERRUPTED,
            }));
        }
    }
    out
}

/// Collect tool-call ids declared by an assistant message.
fn pending_tool_ids(msg: &Value) -> Vec<String> {
    if msg.get("role").and_then(Value::as_str) != Some("assistant") {
        return Vec::new();
    }
    msg.get("tool_calls")
        .and_then(Value::as_array)
        .map(|calls| {
            calls
                .iter()
                .filter_map(|c| c.get("id").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// Ids answered by consecutive `tool` messages starting at `from`.
fn answered_ids(messages: &[Value], from: usize, ids: &[String]) -> Vec<String> {
    let mut found = Vec::new();
    for msg in &messages[from..] {
        if msg.get("role").and_then(Value::as_str) != Some("tool") {
            break;
        }
        if let Some(id) = msg.get("tool_call_id").and_then(Value::as_str)
            && ids.iter().any(|want| want == id)
        {
            found.push(id.to_string());
        }
    }
    found
}
