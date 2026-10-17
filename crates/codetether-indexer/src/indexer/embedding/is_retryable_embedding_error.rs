//! is retryable embedding error for codebase indexing.

pub(in crate::indexer) fn is_retryable_embedding_error(error: &anyhow::Error) -> bool {
    let msg = error.to_string().to_ascii_lowercase();
    [
        "timeout",
        "timed out",
        "connection reset",
        "connection refused",
        "temporary",
        "temporarily unavailable",
        "rate limit",
        "too many requests",
        " 429",
        " 500",
        " 502",
        " 503",
        " 504",
    ]
    .iter()
    .any(|needle| msg.contains(needle))
}
