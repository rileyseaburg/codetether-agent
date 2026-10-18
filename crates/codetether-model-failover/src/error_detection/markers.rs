//! Retry classification markers in their original precedence groups.
pub(super) const TRANSIENT_STATUS_CODES: &[&str] = &[
    " 429 ", " 500 ", " 502 ", " 503 ", " 504 ", " 520 ", " 521 ", " 522 ", " 523 ", " 524 ",
    " 525 ", " 526 ", " 529 ", " 598 ", " 599 ", " 798 ",
];
pub(super) const NON_RETRYABLE_STATUS_CODES: &[&str] =
    &[" 400 ", " 401 ", " 403 ", " 404 ", " 422 "];
pub(super) const MARKERS: &[&str] = &[
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
    "servers are currently overloaded",
    "server_is_overloaded",
];
