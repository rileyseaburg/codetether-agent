//! Provider error classification helpers.

/// Returns true when an upstream provider rejected the prompt as too large.
pub fn is_prompt_too_long_message(message: &str) -> bool {
    let msg = message.to_ascii_lowercase();
    msg.contains("prompt is too long")
        || msg.contains("context length")
        || msg.contains("maximum context")
        || msg.contains("context window")
        || (msg.contains("tokens") && msg.contains("maximum") && msg.contains("prompt"))
}

/// Returns true when an upstream provider error can be retried or failed over.
pub fn is_retryable_upstream_message(message: &str) -> bool {
    retry_markers()
        .iter()
        .any(|needle| message.contains(needle))
}

fn retry_markers() -> [&'static str; 21] {
    [
        " 500 ",
        " 504 ",
        " 502 ",
        " 429 ",
        "status code 500",
        "status code 504",
        "status code 502",
        "status code 429",
        "internal server error",
        "gateway timeout",
        "upstream request timeout",
        "server error: 500",
        "server error: 504",
        "network error",
        "connection reset",
        "connection refused",
        "timed out",
        "timeout",
        "broken pipe",
        "unexpected eof",
        "context window",
    ]
}
