impl OpenAiCodexProvider {
    fn record_incomplete_response(event: &Value, chunks: &mut Vec<StreamChunk>) {
        let reason = event
            .pointer("/response/incomplete_details/reason")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        chunks.push(StreamChunk::Error(format!(
            "codex-retryable: Incomplete response returned, reason: {reason}"
        )));
    }
}
