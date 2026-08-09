//! Bedrock pairing invariant: every toolUse is answered in the next message
//! and no toolResult is an orphan.

use serde_json::Value;

#[path = "invariant/orphan.rs"]
mod orphan;
#[path = "invariant/parts.rs"]
mod parts;

pub(in crate::provider::bedrock::convert::repair::tests) use parts::{call, result, text};

/// Returns a description of the first Bedrock pairing violation, if any.
pub(super) fn violation(messages: &[Value]) -> Option<String> {
    for (index, message) in messages.iter().enumerate() {
        let ids = ids_of(message, "/toolUse/toolUseId");
        let results = ids_of(message, "/toolResult/toolUseId");
        if !ids.is_empty() && !results.is_empty() {
            return Some(format!("message {index} mixes toolUse and toolResult"));
        }
        if ids.is_empty() {
            continue;
        }
        let answered = messages
            .get(index + 1)
            .map(|next| ids_of(next, "/toolResult/toolUseId"))
            .unwrap_or_default();
        if let Some(missing) = ids.iter().find(|id| !answered.contains(id)) {
            return Some(format!(
                "message {index} toolUse {missing} unanswered at messages.{}",
                index + 1
            ));
        }
    }
    orphan::detect(messages)
}

pub(super) fn ids_of(message: &Value, pointer: &str) -> Vec<String> {
    message
        .get("content")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|part| part.pointer(pointer)?.as_str())
        .map(String::from)
        .collect()
}
