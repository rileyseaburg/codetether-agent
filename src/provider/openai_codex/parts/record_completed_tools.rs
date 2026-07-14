impl OpenAiCodexProvider {
    fn record_completed_tools(
        parser: &mut ResponsesSseParser,
        event: &Value,
        chunks: &mut Vec<StreamChunk>,
    ) {
        let Some(output) = event.pointer("/response/output").and_then(Value::as_array) else {
            return;
        };
        for item in output {
            if item.get("type").and_then(Value::as_str) != Some("function_call") {
                continue;
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
}
