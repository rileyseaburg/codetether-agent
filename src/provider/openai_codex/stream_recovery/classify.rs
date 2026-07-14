//! Retry classification for Codex transport and response-event failures.

const PERMANENT: &[&str] = &[
    "bad request",
    "unauthorized",
    "forbidden",
    "invalid api key",
    "context length",
    "content policy",
    " 400 ",
    " 401 ",
    " 403 ",
];

const TRANSIENT: &[&str] = &[
    "you can retry",
    "processing your request",
    "request id",
    "timeout",
    "timed out",
    "connection reset",
    "connection closed",
    "broken pipe",
    "unexpected eof",
    "rate limit",
    "too many requests",
    "service unavailable",
    "bad gateway",
    " 429 ",
    " 500 ",
    " 502 ",
    " 503 ",
    " 504 ",
];

pub(super) fn is_retryable(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    !PERMANENT.iter().any(|marker| lower.contains(marker))
        && TRANSIENT.iter().any(|marker| lower.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::is_retryable;

    #[test]
    fn openai_request_id_error_is_retryable() {
        assert!(is_retryable(
            "An error occurred while processing your request. You can retry your request. Please include the request ID abc"
        ));
    }

    #[test]
    fn permanent_status_wins() {
        assert!(!is_retryable("403 forbidden; request ID abc"));
    }
}
