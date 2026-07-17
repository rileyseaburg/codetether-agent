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
                if !event
                    .get("item")
                    .is_some_and(|item| super::codex_reasoning::push_item_if_reasoning(item, chunks))
                {
                    Self::record_done_tool(parser, event, chunks);
                }
            }
            Some("response.completed") | Some("response.done")
                if event.pointer("/response/status").and_then(Value::as_str)
                    != Some("in_progress") =>
            {
                Self::record_completed_response(parser, event, chunks)
            }
            Some("response.failed") => {
                Self::record_response_error(event, chunks, "Response failed")
            }
            Some("error") => Self::record_response_error(event, chunks, "Realtime error"),
            other => super::codex_reasoning::push_if_reasoning(other, event, chunks),
        }
    }
}
