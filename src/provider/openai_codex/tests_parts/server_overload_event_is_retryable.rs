#[test]
fn server_overload_event_is_retryable() {
    let mut chunks = Vec::new();
    let failed = serde_json::json!({"type": "response.failed", "response": {"error": {
        "code": "server_is_overloaded", "message": "Our servers are currently overloaded"
    }}});
    OpenAiCodexProvider::parse_responses_event(
        &mut ResponsesSseParser::default(), &failed, &mut chunks
    );
    assert!(matches!(&chunks[..], [StreamChunk::Error(message)] if message.starts_with("codex-retryable:")));
}