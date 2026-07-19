impl OpenAiCodexProvider {
    fn parse_responses_event(
        parser: &mut ResponsesSseParser,
        event: &Value,
        chunks: &mut Vec<StreamChunk>,
    ) {
        match event.get("type").and_then(Value::as_str) {
            Some("response.output_text.delta") => Self::record_text_delta(event, chunks),
            Some("response.output_item.added") => {
                if let Some(item) = event.get("item")
                    && !super::codex_reasoning::push_item_if_reasoning(item, chunks)
                {
                    Self::record_responses_tool_item(parser, item, chunks, false);
                }
            }
            Some("response.function_call_arguments.delta") => {
                Self::record_responses_tool_arguments(parser, event, chunks, false);
            }
            Some("response.function_call_arguments.done") => {
                Self::record_responses_tool_arguments(parser, event, chunks, true);
            }
            Some("response.output_item.done") => {
                let item = event.get("item");
                if !item
                    .is_some_and(|item| super::codex_reasoning::push_item_if_reasoning(item, chunks))
                {
                    Self::record_done_tool(parser, event, chunks);
                }
                if let Some(content) = item.and_then(output_item::checkpoint) {
                    chunks.push(StreamChunk::OutputItemDone { content });
                }
            }
            Some("response.completed") => {
                Self::record_completed_response(parser, event, chunks)
            }
            Some("response.failed") => {
                Self::record_response_error(event, chunks, "Response failed")
            }
            Some("response.incomplete") => Self::record_incomplete_response(event, chunks),
            other => super::codex_reasoning::push_if_reasoning(other, event, chunks),
        }
    }
}
