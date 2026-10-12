//! Buffered chat-completion request encoding.
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

pub(crate) fn body(request: &CompletionRequest) -> Value {
    let mut body = json!({
        "model": request.model,
        "messages": crate::messages::convert(&request.messages, false),
        "temperature": request.temperature.unwrap_or(1.0),
        "stream": false,
        "thinking": crate::thinking::thinking_config_for_request(request)
    });
    let tools = tools(request);
    if !tools.is_empty() {
        body["tools"] = json!(tools);
    }
    if let Some(max) = request.max_tokens {
        body["max_tokens"] = json!(max);
    }
    if let Some(top_p) = request.top_p {
        body["top_p"] = json!(top_p);
    }
    if !request.stop.is_empty() {
        body["stop"] = json!(request.stop);
    }
    body
}
