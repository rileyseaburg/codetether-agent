//! Buffered and streaming request body construction.
use super::model_id;
use crate::*;
impl VertexGlmProvider {
    pub(crate) fn body(request: &CompletionRequest, streaming: bool) -> Value {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // Resolve model ID to Vertex format
        let model = model_id::normalize(&request.model);

        // GLM-5 defaults to temperature 1.0 for best results
        let temperature = request.temperature.unwrap_or(1.0);

        let mut body = json!({
            "model": model,
            "messages": messages,
            "temperature": temperature,
            "stream": streaming,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        body
    }
}
