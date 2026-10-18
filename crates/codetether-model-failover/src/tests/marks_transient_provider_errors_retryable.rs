//! Provider-error regression retained from the native smart switch module.
use super::super::*;

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
