//! Tool schema encoding for Z.AI buffered and streaming requests.
use crate::CompletionRequest;
use serde_json::{Value, json};
pub(crate) fn tools(request: &CompletionRequest) -> Vec<Value> {
    request
        .tools
        .iter()
        .map(|tool| {
            json!({
                "type": "function", "function": {
                    "name": tool.name, "description": tool.description,
                    "parameters": tool.parameters
                }
            })
        })
        .collect()
}
