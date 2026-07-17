impl OpenAiCodexProvider {
    fn build_http_responses_body(request: &CompletionRequest) -> Value {
        let (model, reasoning, service_tier) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(&request.model);
        let tools = Self::convert_responses_tools(&request.tools);
        let instructions = Self::extract_responses_instructions(&request.messages);
        let mut body = json!({
            "model": model,
            "instructions": instructions,
            "input": Self::convert_messages_to_responses_input(&request.messages),
            "include": ["reasoning.encrypted_content"],
            "prompt_cache_key": Self::prompt_cache_key(&instructions),
            "stream": true,
            "store": false,
            "tool_choice": "auto",
            "parallel_tool_calls": true,
        });
        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(level) = reasoning {
            body["reasoning"] = json!({ "effort": level.as_wire_str() });
        }
        Self::apply_service_tier(&mut body, service_tier);
        body
    }
}
