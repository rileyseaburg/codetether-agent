//! Shared assistant message assembly for both history formats.
use crate::{ContentPart, Message};
use serde_json::{Value, json};

pub(super) fn convert(
    msg: &Message,
    include_reasoning_content: bool,
    normalize_arguments: &impl Fn(&str) -> String,
) -> Value {
    let text = super::text::text(msg);
    let tool_calls = super::calls::calls(msg, normalize_arguments);
    let mut msg_json = if tool_calls.is_empty() {
        json!({
            "role": "assistant",
            "content": text,
        })
    } else {
        // Z.AI rejects content: "" when tool_calls is present;
        // use null instead (same pattern as openai_codex.rs).
        json!({
            "role": "assistant",
            "content": if text.is_empty() { Value::Null } else { json!(text) },
            "tool_calls": tool_calls,
        })
    };
    if include_reasoning_content {
        let reasoning: String = msg
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Thinking { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");
        if !reasoning.is_empty() {
            msg_json["reasoning_content"] = json!(reasoning);
        }
    }
    msg_json
}
