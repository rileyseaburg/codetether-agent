impl OpenAiCodexProvider {
    fn record_done_tool(
        parser: &mut ResponsesSseParser,
        event: &Value,
        chunks: &mut Vec<StreamChunk>,
    ) {
        let Some(item) = event.get("item") else {
            return;
        };
        if item.get("type").and_then(Value::as_str) != Some("function_call") {
            return;
        }
        let key = item
            .get("id")
            .and_then(Value::as_str)
            .or_else(|| item.get("call_id").and_then(Value::as_str));
        if let Some(key) = key {
            Self::record_responses_tool_item(parser, item, chunks, true);
            Self::finish_responses_tool_call(parser, key, chunks);
        }
    }
}
