//! Completion settings and request flags.
use crate::*;
impl CopilotProvider {
    pub(crate) fn body(request: &CompletionRequest) -> (Value, bool, bool) {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);
        let is_agent = Self::is_agent_initiated(&request.messages);
        let has_vision = Self::has_vision_input(&request.messages);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(top_p) = request.top_p {
            body["top_p"] = json!(top_p);
        }
        if let Some(max) = request.max_tokens {
            let key = if request.model.starts_with("gpt-5") {
                "max_completion_tokens"
            } else {
                "max_tokens"
            };
            body[key] = json!(max);
        }
        if !request.stop.is_empty() {
            body["stop"] = json!(request.stop);
        }
        (body, is_agent, has_vision)
    }
}
