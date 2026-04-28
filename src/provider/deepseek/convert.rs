//! Convert internal [`Message`] to DeepSeek JSON format.
//!
//! Assistant messages with tool calls **must** include `reasoning_content`
//! (even if empty) or the DeepSeek V4 API returns HTTP 400.

use crate::provider::{ContentPart, Message, Role};
use serde_json::{Value, json};

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
    let txt = collect_text(m);
    let reason = collect_thinking(m);
    let calls = collect_calls(m);
    tracing::debug!(text_len = txt.len(), reason_len = reason.len(), tool_calls = calls.len(), "DeepSeek convert assistant msg");
    if calls.is_empty() {
        json!({"role": "assistant", "content": txt})
    } else {
        json!({"role": "assistant", "content": txt, "reasoning_content": reason, "tool_calls": calls})
    }
}

fn text_msg(m: &Message) -> Value {
    let role = match m.role {
        Role::System => "system",
        _ => "user",
    };
    json!({"role": role, "content": collect_text(m)})
}

fn collect_text(m: &Message) -> String {
    m.content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect()
}
fn collect_thinking(m: &Message) -> String {
    m.content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Thinking { text } => Some(text.clone()),
            _ => None,
        })
        .collect()
}
fn collect_calls(m: &Message) -> Vec<Value> {
    m.content.iter().filter_map(|p| match p {
        ContentPart::ToolCall { id, name, arguments, .. } =>
            Some(json!({"id": id, "type": "function", "function": {"name": name, "arguments": arguments}})),
        _ => None,
    }).collect()
}
