//! Build a JSON request body for the raw-SSE reasoning stream.
//!
//! Mirrors the typed `async-openai` request the OpenAI provider would send,
//! but as plain `serde_json` so vendor reasoning fields survive end-to-end.

use serde_json::{Value, json};

use crate::provider::{CompletionRequest, ToolDefinition};

use super::OpenAIProvider;
use super::alias;
use super::sse_msg::messages_json;

impl OpenAIProvider {
    /// Build the streaming chat-completion request body.
    pub(super) fn reasoning_body(&self, request: &CompletionRequest) -> Value {
        let model = alias::normalize_model_id(&self.provider_name, &request.model);
        let mut body = json!({
            "model": model.as_ref(),
            "messages": messages_json(&request.messages),
            "stream": true,
            "stream_options": { "include_usage": true },
        });
        let tools = tools_json(&request.tools);
        if !tools.is_empty() {
            body["tools"] = Value::Array(tools);
        }
        if let Some(t) = request.temperature {
            body["temperature"] = json!(t);
        }
        if let Some(p) = request.top_p {
            body["top_p"] = json!(p);
        }
        if let Some(max) = request.max_tokens {
            body["max_completion_tokens"] = json!(max);
        }
        body
    }
}

fn tools_json(tools: &[ToolDefinition]) -> Vec<Value> {
    tools
        .iter()
        .map(|t| {
            json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters,
                }
            })
        })
        .collect()
}
