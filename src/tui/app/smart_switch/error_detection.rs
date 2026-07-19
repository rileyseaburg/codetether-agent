//! Provider error classification and normalization.

#[path = "error_detection/normalization.rs"]
mod normalization;
pub use normalization::{normalize_provider_alias, smart_switch_model_key};

/// Classifies provider errors as retryable (rate limit, timeout, 5xx) vs permanent.
pub fn is_retryable_provider_error(err: &str) -> bool {
    let normalized = err.to_ascii_lowercase();
    let transient_status_codes = [
        " 429 ", " 500 ", " 502 ", " 503 ", " 504 ", " 520 ", " 521 ", " 522 ", " 523 ", " 524 ",
        " 525 ", " 526 ", " 529 ", " 598 ", " 599 ", " 798 ",
    ];
    let non_retryable_status_codes = [" 400 ", " 401 ", " 403 ", " 404 ", " 422 "];
    let markers = [
        "429",
        "rate limit",
        "too many requests",
        "quota exceeded",
        "token_quota_exceeded",
        "too many tokens",
        "service unavailable",
        "temporarily unavailable",
        "bad gateway",
        "gateway timeout",
        "timed out",
        "timeout",
        "connection reset",
        "connection refused",
        "network error",
        "unknown api error",
        "api_error",
        "unknown error",
        "protocol error code 469",
        "no text payload",
        "context length",
        "context window",
    ];

    if normalized.contains("retry limit exhausted")
        || non_retryable_status_codes
            .iter()
            .any(|c| normalized.contains(c))
    {
        return false;
    }

    markers.iter().any(|m| normalized.contains(m))
        || transient_status_codes
            .iter()
            .any(|c| normalized.contains(c))
}
