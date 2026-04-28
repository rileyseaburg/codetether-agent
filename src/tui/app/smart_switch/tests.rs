//! Tests for error detection helpers.

use super::*;

#[test]
fn marks_transient_provider_errors_retryable() {
    let errors = [
        "OpenAI API error (429 Too Many Requests): rate limit exceeded",
        "Gemini returned protocol error code 469 with no text payload",
        "Anthropic API error: unknown error, 520 (1000) (Some(\"api_error\"))",
        "Anthropic API error: unknown error, 798 (1000) (Some(\"api_error\"))",
        "context length exceeded, maximum context length is 128k",
        "Your input exceeds the context window of this model",
    ];
    assert!(errors.iter().all(|err| is_retryable_provider_error(err)));
}

#[test]
fn ignores_non_retryable_bad_request_errors() {
    assert!(!is_retryable_provider_error(
        "OpenAI API error (400 Bad Request): Instructions are required"
    ));
    assert!(!is_retryable_provider_error(
        "OpenAI API error (401 Unauthorized): Invalid API key"
    ));
    assert!(!is_retryable_provider_error(
        "OpenAI API error (403 Forbidden): Access denied"
    ));
    assert!(!is_retryable_provider_error(
        "OpenAI API error (404 Not Found): Model not found"
    ));
    assert!(!is_retryable_provider_error(
        "OpenAI API error (422 Unprocessable Entity): Invalid request"
    ));
}

#[test]
fn normalizes_provider_aliases() {
    assert_eq!(normalize_provider_alias("z-ai"), "zai");
    assert_eq!(normalize_provider_alias("ZhipuAI"), "zai");
    assert_eq!(normalize_provider_alias("openai"), "openai");
}

#[test]
fn model_key_normalizes() {
    assert_eq!(
        smart_switch_model_key("  Minimax-Credits/MiniMax-M2.5-Highspeed "),
        "minimax-credits/minimax-m2.5-highspeed".to_string()
    );
}
