#[test]
fn done_sentinel_is_activity_not_completion() {
    let mut parser = ResponsesSseParser::default();
    let chunks = OpenAiCodexProvider::parse_responses_sse_bytes(&mut parser, b"data: [DONE]\n\n");
    assert!(chunks.is_empty());
}

#[test]
fn completed_event_requires_response_id() {
    let mut parser = ResponsesSseParser::default();
    let event = json!({"type": "response.completed", "response": {"status": "completed"}});
    let mut chunks = Vec::new();
    OpenAiCodexProvider::parse_responses_event(&mut parser, &event, &mut chunks);
    assert!(matches!(&chunks[..], [StreamChunk::Error(message)] if message.contains("missing response id")));
}

#[test]
fn incomplete_event_requests_stream_retry() {
    let mut parser = ResponsesSseParser::default();
    let event = json!({"type": "response.incomplete", "response": {"incomplete_details": {"reason": "max_output_tokens"}}});
    let mut chunks = Vec::new();
    OpenAiCodexProvider::parse_responses_event(&mut parser, &event, &mut chunks);
    assert!(matches!(&chunks[..], [StreamChunk::Error(message)] if message.starts_with("codex-retryable:")));
}

#[test]
fn bare_error_event_is_ignored_but_context_failure_is_permanent() {
    let mut parser = ResponsesSseParser::default();
    let mut chunks = Vec::new();
    OpenAiCodexProvider::parse_responses_event(&mut parser, &json!({"type": "error"}), &mut chunks);
    assert!(chunks.is_empty());
    let failed = json!({"type": "response.failed", "response": {"error": {"code": "context_length_exceeded", "message": "too long"}}});
    OpenAiCodexProvider::parse_responses_event(&mut parser, &failed, &mut chunks);
    assert!(matches!(&chunks[..], [StreamChunk::Error(message)] if message.starts_with("codex-permanent:")));
}

#[test]
fn wrapped_websocket_error_shape_is_ignored_by_sse_parser() {
    let mut chunks = Vec::new();
    OpenAiCodexProvider::parse_responses_event(
        &mut ResponsesSseParser::default(),
        &json!({"type": "error", "status": 400, "error": {"message": "bad"}}),
        &mut chunks,
    );
    assert!(chunks.is_empty());
}