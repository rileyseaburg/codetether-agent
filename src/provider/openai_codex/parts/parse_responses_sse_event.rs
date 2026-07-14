impl OpenAiCodexProvider {
    fn parse_responses_sse_event(
        parser: &mut ResponsesSseParser,
        data: &str,
        chunks: &mut Vec<StreamChunk>,
    ) {
        if data == "[DONE]" {
            chunks.push(StreamChunk::Done { usage: None });
            return;
        }

        let Ok(event): Result<Value, _> = serde_json::from_str(data) else {
            return;
        };

        Self::parse_responses_event(parser, &event, chunks);
    }
}
