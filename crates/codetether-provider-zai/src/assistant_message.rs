//! Assistant history encoding, including tool-only and reasoning turns.
use crate::{ContentPart, Message};
use serde_json::{Value, json};

pub(crate) fn convert(message: &Message, include_reasoning: bool) -> Value {
    let text = message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<String>();
    let calls: Vec<Value> = message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => Some(json!({
                "id": id, "type": "function", "function": {
                    "name": name, "arguments": crate::tool_arguments::normalize(arguments)
                }
            })),
            _ => None,
        })
        .collect();
    let mut result = if calls.is_empty() {
        json!({"role": "assistant", "content": text})
    } else {
        json!({"role": "assistant", "content": if text.is_empty() {
            Value::Null
        } else { json!(text) }, "tool_calls": calls})
    };
    if include_reasoning {
        let reasoning = message
            .content
            .iter()
            .filter_map(|part| match part {
                ContentPart::Thinking { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect::<String>();
        if !reasoning.is_empty() {
            result["reasoning_content"] = json!(reasoning);
        }
    }
    result
}
