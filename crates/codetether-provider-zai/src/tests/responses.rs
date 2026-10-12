//! Buffered completion and usage conversion regressions.
use crate::{ContentPart, FinishReason};
use serde_json::json;

#[test]
fn completion_preserves_tools_and_separates_cached_usage() {
    let response = crate::response::parse(
        &json!({
            "choices":[{"message":{"content":"Using a tool.", "reasoning_content":"Plan",
                "tool_calls":[{"id":"call_1", "function":{"name":"weather",
                    "arguments":{"city":"Beijing"}}}]}, "finish_reason":"stop"}],
            "usage":{"prompt_tokens":120,"completion_tokens":45,"total_tokens":165,
                "prompt_tokens_details":{"cached_tokens":64}}
        })
        .to_string(),
    )
    .unwrap();
    assert_eq!(response.finish_reason, FinishReason::ToolCalls);
    assert_eq!(response.usage.prompt_tokens, 56);
    assert_eq!(response.usage.cache_read_tokens, Some(64));
    assert_eq!(response.usage.total_tokens, 165);
    assert!(response.message.content.iter().any(|part| matches!(part,
        ContentPart::ToolCall { arguments, .. } if arguments == r#"{"city":"Beijing"}"#)));
}

#[test]
fn response_parse_errors_are_explicit() {
    assert!(crate::response::parse("not JSON").is_err());
    assert!(crate::response::parse(r#"{"choices":[]}"#).is_err());
}

#[test]
fn preview_respects_unicode_boundaries() {
    assert_eq!(crate::preview::text("a😀b", 2), "a😀");
}
