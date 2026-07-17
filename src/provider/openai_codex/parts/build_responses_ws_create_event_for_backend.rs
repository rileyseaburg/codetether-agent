impl OpenAiCodexProvider {
    fn build_responses_ws_create_event_for_backend(
        request: &CompletionRequest,
        model: &str,
        reasoning_effort: Option<ThinkingLevel>,
        service_tier: Option<CodexServiceTier>,
        backend: ResponsesWsBackend,
    ) -> Value {
        let instructions = Self::extract_responses_instructions(&request.messages);
        let input = Self::convert_messages_to_responses_input(&request.messages);
        let tools = Self::convert_responses_tools(&request.tools);

        let mut event = json!({
            "type": "response.create",
            "model": model,
            "store": false,
            "instructions": instructions,
            "input": input,
            "include": ["reasoning.encrypted_content"],
            "prompt_cache_key": Self::prompt_cache_key(&instructions),
        });

        if !tools.is_empty() {
            event["tools"] = json!(tools);
        }
        if let Some(level) = reasoning_effort {
            event["reasoning"] = json!({ "effort": level.as_wire_str() });
        }
        Self::apply_service_tier(&mut event, service_tier);
        if backend == ResponsesWsBackend::OpenAi {
            event["tool_choice"] = json!("auto");
            event["parallel_tool_calls"] = json!(true);
        }
        if backend == ResponsesWsBackend::OpenAi
            && let Some(max_tokens) = request.max_tokens
        {
            event["max_output_tokens"] = json!(max_tokens);
        }

        event
    }
}
