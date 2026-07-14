impl OpenAiCodexProvider {
    fn record_responses_tool_item(
        parser: &mut ResponsesSseParser,
        item: &Value,
        chunks: &mut Vec<StreamChunk>,
        include_arguments: bool,
    ) -> Option<String> {
        if item.get("type").and_then(Value::as_str) != Some("function_call") {
            return None;
        }
        let key = item
            .get("id")
            .and_then(Value::as_str)
            .or_else(|| item.get("call_id").and_then(Value::as_str))?;
        let call_id = item
            .get("call_id")
            .and_then(Value::as_str)
            .or_else(|| item.get("id").and_then(Value::as_str))?
            .to_string();
        let name = item.get("name").and_then(Value::as_str).map(str::to_string);
        let arguments = include_arguments
            .then(|| Self::extract_json_string(item.get("arguments")))
            .flatten();
        let state = Self::response_tool_state(parser, key, call_id);
        Self::start_response_tool(state, name, chunks);
        if let Some(arguments) = arguments {
            Self::emit_missing_responses_tool_arguments(state, arguments, chunks);
        }
        Some(state.call_id.clone())
    }
}
