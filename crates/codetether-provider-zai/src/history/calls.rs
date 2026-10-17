//! Assistant tool-call content and argument normalization.
use crate::{ContentPart, Message};
use serde_json::{Value, json};

pub(super) fn calls(msg: &Message, normalize_arguments: &impl Fn(&str) -> String) -> Vec<Value> {
    let tool_calls: Vec<Value> = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => {
                let args_string = normalize_arguments(arguments);
                Some(json!({
                    "id": id,
                    "type": "function",
                    "function": {
                        "name": name,
                        "arguments": args_string
                    }
                }))
            }
            _ => None,
        })
        .collect();

    tool_calls
}
