//! Classification applied only to non-success response bodies or errors.

pub(super) fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(
        status.as_u16(),
        429 | 500 | 502 | 503 | 504 | 520 | 522 | 524
    )
}

pub(super) fn is_permanent_message(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("has expired")
        || lower.contains("package has expired")
        || lower.contains("renewing the subscription")
        || lower.contains("renew the subscription")
        || lower.contains("subscription has expired")
        || lower.contains("\"1309\"")
        || lower.contains("code\":1309")
        || lower.contains("insufficient balance")
        || lower.contains("account has been suspended")
}

pub(super) fn is_retryable_message(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("temporarily overloaded")
        || lower.contains("rate limit")
        || lower.contains("connection reset")
        || lower.contains("connection closed")
        || lower.contains("service unavailable")
        || lower.contains("bad gateway")
        || lower.contains("token_quota_exceeded")
        || lower.contains("too many tokens")
        || lower.contains("limit exceeded")
        || lower.contains("quota exceeded")
        || lower.contains("too many requests")
}
