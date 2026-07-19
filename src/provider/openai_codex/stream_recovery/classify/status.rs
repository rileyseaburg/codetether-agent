//! HTTP status and exhausted-request stream retry classification.

const PERMANENT_OVERLOAD_CODES: &[&str] = &["server_is_overloaded", "slow_down"];

pub(super) fn terminal_stream_retryable(error: &anyhow::Error) -> bool {
    let message = format!("{error:#}").to_ascii_lowercase();
    status_code(&message).is_some_and(|status| !matches!(status, 400 | 429))
}

pub(super) fn exhausted_request_retryable(reason: &str) -> bool {
    let lower = reason.to_ascii_lowercase();
    !PERMANENT_OVERLOAD_CODES
        .iter()
        .any(|code| lower.contains(code))
}

pub(super) fn tag_stream_open(error: anyhow::Error) -> anyhow::Error {
    let message = format!("{error:#}");
    let lower = message.to_ascii_lowercase();
    let Some(status) = status_code(&lower) else {
        return error;
    };
    let prefix = if matches!(status, 400 | 429) {
        "codex-permanent"
    } else {
        "codex-retryable"
    };
    anyhow::anyhow!("{prefix}: {message}")
}

fn status_code(message: &str) -> Option<u16> {
    ["openai api error (", "http error: ", "http status "]
        .iter()
        .find_map(|marker| message.split(marker).nth(1)?.get(..3)?.parse().ok())
}
