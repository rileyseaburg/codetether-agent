use super::event;
use crate::provider::{StreamChunk, openai_codex::ResponsesSseParser};
use serde_json::json;

#[test]
fn websocket_connection_limit_is_retryable() {
    let parsed = event(
        &mut ResponsesSseParser::default(),
        &json!({"type": "error", "error": {
            "code": "websocket_connection_limit_reached", "message": "limit"
        }}),
    );
    assert!(
        matches!(&parsed.chunks[..], [StreamChunk::Error(message)] if message.starts_with("codex-retryable:"))
    );
}

#[test]
fn websocket_bad_request_is_permanent() {
    let parsed = event(
        &mut ResponsesSseParser::default(),
        &json!({"type": "error", "status": 400, "error": {"message": "bad"}}),
    );
    assert!(
        matches!(&parsed.chunks[..], [StreamChunk::Error(message)] if message.starts_with("codex-permanent:"))
    );
}
