//! Non-streaming prompt-cache accounting regressions.
use super::{AnthropicUsage, usage};

#[test]
fn total_includes_cached_input_without_double_charging_prompt() {
    let wire: AnthropicUsage = serde_json::from_value(serde_json::json!({
        "input_tokens":10, "output_tokens":5,
        "cache_read_input_tokens":100, "cache_creation_input_tokens":20,
    }))
    .unwrap();
    let result = usage(Some(&wire));
    assert_eq!(result.prompt_tokens, 10);
    assert_eq!(result.total_tokens, 135);
    assert_eq!(result.cache_read_tokens, Some(100));
    assert_eq!(result.cache_write_tokens, Some(20));
}

#[test]
fn absent_usage_is_zero_and_cache_is_unknown() {
    assert_eq!(usage(None).total_tokens, 0);
    assert_eq!(usage(None).cache_read_tokens, None);
}
