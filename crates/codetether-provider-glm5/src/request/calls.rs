//! Assistant function call encoding.
use super::arguments;
use crate::*;
pub(super) fn encode(msg: &Message) -> Vec<Value> {
    msg.content
        .iter()
        .filter_map(|p| match p {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => {
                // vLLM expects arguments as a JSON string
                let args_string = arguments::encode(arguments);
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
        .collect()
}
