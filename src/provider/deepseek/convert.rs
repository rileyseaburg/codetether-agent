//! Convert internal [`Message`] to DeepSeek JSON format.
//!
//! Assistant messages with tool calls **must** include
//! `reasoning_content` (per DeepSeek V4 docs) or the API returns
//! HTTP 400. We send `null` for empty fields to match the spec.

use crate::provider::{ContentPart, Message, Role};
use serde_json::{Value, json};

use super::convert_helpers;

pub(crate) fn messages(msgs: &[Message]) -> Vec<Value> {
    msgs.iter()
        .map(|m| match m.role {
            Role::Tool => tool_msg(m),
            Role::Assistant => assistant_msg(m),
            _ => text_msg(m),
        })
        .collect()
}

fn tool_msg(m: &Message) -> Value {
    match m.content.first() {
        Some(ContentPart::ToolResult {
            tool_call_id,
            content,
        }) => json!({"role": "tool", "tool_call_id": tool_call_id, "content": content}),
        _ => json!({"role": "tool", "content": ""}),
    }
}

fn assistant_msg(m: &Message) -> Value {
    let txt = convert_helpers::collect_text(m);
    let calls = convert_helpers::collect_calls(m);
    let reason = convert_helpers::collect_thinking(m);
    tracing::debug!(tool_calls = calls.len(), "DeepSeek convert assistant msg");
    if calls.is_empty() {
        let mut val = json!({"role": "assistant", "content": txt});
        if !reason.is_empty() {
            val["reasoning_content"] = json!(reason);
        }
        val
    } else {
        json!({
            "role": "assistant",
            "content": if txt.is_empty() { Value::Null } else { json!(txt) },
            "reasoning_content": if reason.is_empty() { Value::Null } else { json!(reason) },
            "tool_calls": calls
        })
    }
}

fn text_msg(m: &Message) -> Value {
    let role = match m.role {
        Role::System => "system",
        _ => "user",
    };
    json!({"role": role, "content": convert_helpers::collect_text(m)})
}
