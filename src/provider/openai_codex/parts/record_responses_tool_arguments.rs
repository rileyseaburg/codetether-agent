impl OpenAiCodexProvider {
    fn record_responses_tool_arguments(
        parser: &mut ResponsesSseParser,
        event: &Value,
        chunks: &mut Vec<StreamChunk>,
        use_final_arguments: bool,
    ) {
        let key = event
            .get("item_id")
            .and_then(Value::as_str)
            .or_else(|| event.get("call_id").and_then(Value::as_str));
        let Some(key) = key else { return };
        let call_id = event
            .get("call_id")
            .and_then(Value::as_str)
            .unwrap_or(key)
            .to_string();
        let name = event
            .get("name")
            .and_then(Value::as_str)
            .map(str::to_string);
        let arguments = Self::response_event_arguments(event, use_final_arguments);
        let state = Self::response_tool_state(parser, key, call_id);
        Self::start_response_tool(state, name, chunks);
        if let Some(arguments) = arguments {
            Self::emit_missing_responses_tool_arguments(state, arguments, chunks);
        }
    }
}
