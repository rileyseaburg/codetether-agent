//! Migrated application stream regression.
use super::*;
#[test]
fn stream_response_parses_usage_block() {
    let data = r#"{"choices":[],"usage":{"prompt_tokens":120,"completion_tokens":45,"total_tokens":165,"prompt_tokens_details":{"cached_tokens":64}}}"#;
    let parsed: ZaiStreamResponse = serde_json::from_str(data).expect("usage chunk should parse");
    let usage = parsed.usage.expect("usage present");
    assert_eq!(usage.prompt_tokens, 120);
    assert_eq!(usage.completion_tokens, 45);
    assert_eq!(usage.total_tokens, 165);
    assert_eq!(
        usage.prompt_tokens_details.map(|d| d.cached_tokens),
        Some(64)
    );
}
