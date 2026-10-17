//! Existing output_budget regression coverage.

use super::effective_max_tokens;

#[test]
fn fable_gets_thinking_floor() {
    assert_eq!(
        effective_max_tokens(Some(8_192), "global.anthropic.claude-fable-5"),
        32_000
    );
    assert_eq!(
        effective_max_tokens(Some(64_000), "global.anthropic.claude-fable-5"),
        64_000
    );
    assert_eq!(
        effective_max_tokens(Some(8_192), "us.anthropic.claude-opus-4-8"),
        32_000
    );
}

#[test]
fn non_thinking_models_keep_requested_budget() {
    assert_eq!(
        effective_max_tokens(Some(8_192), "anthropic.claude-3-haiku-20240307-v1:0"),
        8_192
    );
    assert_eq!(effective_max_tokens(None, "amazon.nova-pro-v1:0"), 8_192);
}
