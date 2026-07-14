impl OpenAiCodexProvider {
    fn record_response_error(event: &Value, chunks: &mut Vec<StreamChunk>, fallback: &str) {
        let response_error = event
            .get("response")
            .and_then(|response| response.get("error"))
            .and_then(|error| error.get("message"));
        let message = response_error
            .or_else(|| event.get("error").and_then(|error| error.get("message")))
            .or_else(|| event.get("message"))
            .and_then(Value::as_str)
            .unwrap_or(fallback)
            .to_string();
        chunks.push(StreamChunk::Error(message));
    }
}
