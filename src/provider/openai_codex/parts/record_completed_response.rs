impl OpenAiCodexProvider {
    fn record_completed_response(
        parser: &mut ResponsesSseParser,
        event: &Value,
        chunks: &mut Vec<StreamChunk>,
    ) {
        let Some(response) = event.get("response") else {
            return;
        };
        if !response.get("id").is_some_and(Value::is_string) {
            chunks.push(StreamChunk::Error(
                "codex-retryable: failed to parse ResponseCompleted: missing response id".into(),
            ));
            return;
        }
        Self::record_completed_tools(parser, event, chunks);
        Self::record_completed_reasoning(Some(response), chunks);
        let usage = response
            .get("usage")
            .map(|usage| Self::parse_responses_usage(Some(usage)));
        chunks.push(StreamChunk::Done { usage });
    }
}
