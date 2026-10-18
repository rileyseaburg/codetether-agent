//! Existing TUI regression cases moved beside their implementation.
use crate::local_model::normalize_local_model_ref;
use crate::{is_retryable_provider_error, normalize_provider_alias, smart_switch_model_key};

#[test]
fn normalize_local_model_ref_adds_prefix() {
    assert_eq!(
        normalize_local_model_ref("qwen3.5-9b"),
        "local_cuda/qwen3.5-9b"
    );
    assert_eq!(
        normalize_local_model_ref("local_cuda/qwen3.5-9b"),
        "local_cuda/qwen3.5-9b"
    );
}

#[test]
fn smart_switch_marks_transient_provider_errors_retryable() {
    assert!(is_retryable_provider_error(
        "OpenAI API error (429 Too Many Requests): rate limit exceeded"
    ));
    assert!(is_retryable_provider_error(
        "Gemini returned protocol error code 469 with no text payload"
    ));
    assert!(is_retryable_provider_error(
        "Anthropic API error: unknown error, 520 (1000) (Some(\"api_error\"))"
    ));
    assert!(is_retryable_provider_error(
        "Anthropic API error: unknown error, 798 (1000) (Some(\"api_error\"))"
    ));
}

#[test]
fn smart_switch_ignores_non_retryable_bad_request_errors() {
    assert!(!is_retryable_provider_error(
        "OpenAI API error (400 Bad Request): Instructions are required"
    ));
}

#[test]
fn smart_switch_normalizes_provider_aliases_and_model_keys() {
    assert_eq!(normalize_provider_alias("z-ai"), "zai");
    assert_eq!(
        smart_switch_model_key("  Minimax-Credits/MiniMax-M2.5-Highspeed "),
        "minimax-credits/minimax-m2.5-highspeed".to_string()
    );
}
