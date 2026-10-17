//! Buffered and streaming request fields.
use crate::*;
impl VertexAnthropicProvider {
    pub(crate) fn buffered_body(request: &CompletionRequest) -> Value {
        let (system_prompt, messages) = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // Build request body - note: model is NOT in body for Vertex, it's in URL
        let mut body = json!({
            "anthropic_version": VERTEX_ANTHROPIC_VERSION,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(8192),
        });

        if let Some(system) = system_prompt {
            body["system"] = system;
        }
        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(top_p) = request.top_p {
            body["top_p"] = json!(top_p);
        }

        body
    }
    pub(crate) fn stream_body(request: &CompletionRequest) -> Value {
        let (system_prompt, messages) = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let mut body = json!({
            "anthropic_version": VERTEX_ANTHROPIC_VERSION,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(8192),
            "stream": true,
        });

        if let Some(system) = system_prompt {
            body["system"] = system;
        }
        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }

        body
    }
}
