//! Retry classification for Codex transport and response-event failures.

const PERMANENT: &[&str] = &[
    "bad request",
    "unauthorized",
    "forbidden",
    "not found",
    "unprocessable",
    "invalid api key",
    "context length",
    "content policy",
    " 400 ",
    " 401 ",
    " 403 ",
    " 404 ",
    " 422 ",
];

const TRANSIENT: &[&str] = &[
    "you can retry",
    "processing your request",
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
    "connection refused",
    "429 too many requests",
    "500 internal server error",
    "502 bad gateway",
    "503 service unavailable",
    "504 gateway timeout",
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
#[path = "classify/tests.rs"]
mod tests;
