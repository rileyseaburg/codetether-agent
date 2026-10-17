//! Google generation settings and optional fields.
use crate::*;
impl GoogleProvider {
    pub(crate) fn build_body(request: &CompletionRequest) -> Value {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
        });

        if let Some(max_tokens) = request.max_tokens {
            body["max_tokens"] = json!(max_tokens);
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
}
