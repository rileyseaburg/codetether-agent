//! Tests for native Anthropic Messages response parsing.

use crate::provider::ContentPart;
use crate::provider::bedrock::invoke::response::parse_anthropic_messages_response;

#[test]
fn parses_anthropic_messages_response() {
    let raw = r#"{
        "content":[{"type":"thinking","thinking":"","signature":"sig"},{"type":"text","text":"ok"}],
        "stop_reason":"end_turn",
        "usage":{"input_tokens":2,"output_tokens":3,"cache_read_input_tokens":1}
    }"#;
    let parsed = parse_anthropic_messages_response(raw).unwrap();
    assert!(
        matches!(parsed.message.content.last(), Some(ContentPart::Text { text }) if text == "ok")
    );
    assert_eq!(parsed.usage.total_tokens, 5);
    assert_eq!(parsed.usage.cache_read_tokens, Some(1));
}
