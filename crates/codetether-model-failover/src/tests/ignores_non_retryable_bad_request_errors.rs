//! Provider-error regression retained from the native smart switch module.
use super::super::*;

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
    assert!(!is_retryable_provider_error(
        "stream retry limit exhausted after 5 retries: service unavailable"
    ));
}
