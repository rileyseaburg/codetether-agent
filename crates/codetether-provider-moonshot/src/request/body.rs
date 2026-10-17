//! K2 temperature, thinking, and generation settings.
use crate::*;
impl MoonshotProvider {
    pub(crate) fn build_body(request: &CompletionRequest) -> Value {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // Kimi K2.5 requires specific temperatures:
        // - temperature = 1.0 when thinking is enabled
        // - temperature = 0.6 when thinking is disabled
        let temperature = if request.model.contains("k2") {
            0.6 // We disable thinking for tool calling workflows
        } else {
            request.temperature.unwrap_or(0.7)
        };

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "temperature": temperature,
        });

        // Disable thinking mode to avoid needing to track reasoning_content
        // across message roundtrips (required for K2.5)
        if request.model.contains("k2") {
            body["thinking"] = json!({"type": "disabled"});
        }

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        body
    }
}
