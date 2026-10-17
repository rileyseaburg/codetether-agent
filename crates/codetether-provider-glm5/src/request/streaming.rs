//! streaming request body and normalized model.
use crate::*;
impl Glm5Provider {
    pub(crate) fn streaming_body(request: &CompletionRequest) -> (Value, String) {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let temperature = request.temperature.unwrap_or(1.0);
        let model = Self::normalize_model(&request.model);

        let mut body = json!({
            "model": model,
            "messages": messages,
            "temperature": temperature,
            "stream": true,
            "stream_options": { "include_usage": true },
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        (body, model)
    }
}
