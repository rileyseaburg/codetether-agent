//! Tests for Gemini Web's explicitly estimated usage telemetry.

use super::estimate;

#[test]
fn estimates_prompt_and_answer_tokens() {
    let usage = estimate("12345678", "abcd", &[]);
    assert_eq!(usage.prompt_tokens, 2);
    assert_eq!(usage.completion_tokens, 1);
    assert_eq!(usage.total_tokens, 3);
}

#[test]
fn tool_calls_count_as_completion_output() {
    let calls = vec![("read".to_string(), r#"{"path":"README.md"}"#.to_string())];
    let usage = estimate("prompt", "", &calls);
    assert!(usage.prompt_tokens > 0);
    assert!(usage.completion_tokens > 0);
    assert_eq!(
        usage.total_tokens,
        usage.prompt_tokens + usage.completion_tokens
    );
}

#[test]
fn empty_content_remains_zero() {
    let usage = estimate("", "", &[]);
    assert_eq!(usage.prompt_tokens, 0);
    assert_eq!(usage.completion_tokens, 0);
    assert_eq!(usage.total_tokens, 0);
}
