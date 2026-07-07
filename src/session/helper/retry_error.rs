//! Retryable upstream error classification.

const RETRYABLE_NEEDLES: &[&str] = &[
    " 500 ",
    " 503 ",
    " 504 ",
    " 502 ",
    " 429 ",
    "status code 500",
    "status code 503",
    "status code 504",
    "status code 502",
    "status code 429",
    "internal server error",
    "service unavailable",
    "gateway timeout",
    "upstream request timeout",
    "server error: 500",
    "server error: 503",
    "server error: 504",
    "network error",
    "connection reset",
    "connection refused",
    "timed out",
    "timeout",
    "broken pipe",
    "unexpected eof",
    "flagged for possible",
    "content moderation",
    "content policy",
    "safety system",
    "refused to generate",
    // Cerebras rate-limit / quota errors
    "token_quota_exceeded",
    "too many tokens",
    "limit exceeded",
    "rate_limit",
    "ratelimit",
    "quota exceeded",
    "too many requests",
    // Empty-response errors: provider returned 200 but with no content/choices.
    // Happens under model capacity pressure or brief upstream hiccups; always
    // retryable because the request itself was valid.
    "no choices",
    "empty response",
    "stream ended without producing any content",
    "stream ended without assistant content",
];

/// Returns true when an upstream provider error is worth retrying.
pub fn is_retryable_upstream_error(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_ascii_lowercase();
    RETRYABLE_NEEDLES.iter().any(|needle| msg.contains(needle))
}
