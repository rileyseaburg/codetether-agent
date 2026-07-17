impl OpenAiCodexProvider {
    fn record_completed_response(
        parser: &mut ResponsesSseParser,
        event: &Value,
        chunks: &mut Vec<StreamChunk>,
    ) {
        Self::record_completed_tools(parser, event, chunks);
        let response = event.get("response");
        let status = response
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str);
        if matches!(status, Some("failed" | "cancelled" | "incomplete")) {
            Self::record_response_error(event, chunks, "Response failed");
            return;
        }
        Self::record_completed_reasoning(response, chunks);
        let usage = response
            .and_then(|value| value.get("usage"))
            .map(|usage| Self::parse_responses_usage(Some(usage)));
        chunks.push(StreamChunk::Done { usage });
    }
}
