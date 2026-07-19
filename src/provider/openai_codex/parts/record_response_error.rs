impl OpenAiCodexProvider {
    fn record_response_error(event: &Value, chunks: &mut Vec<StreamChunk>, fallback: &str) {
        chunks.push(StreamChunk::Error(event_error::format(event, fallback)));
    }
}
