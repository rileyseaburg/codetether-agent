//! Retry classification for pre-stream HTTP request failures.

#[path = "classify/auth.rs"]
mod auth;
pub(in crate::provider::openai_codex) use auth::{is_unauthorized, is_upgrade_required};
#[path = "classify/status.rs"]
mod status;

const NETWORK: &[&str] = &[
    "timeout",
    "timed out",
    "connection reset",
    "connection closed",
    "broken pipe",
    "unexpected eof",
    "connection refused",
    "network error",
    "dns error",
];

pub(super) fn is_retryable_request(error: &anyhow::Error) -> bool {
    if error
        .chain()
        .any(|cause| cause.downcast_ref::<reqwest::Error>().is_some())
    {
        return true;
    }
    let message = format!("{error:#}").to_ascii_lowercase();
    server_status(&message) || NETWORK.iter().any(|marker| message.contains(marker))
}

fn server_status(message: &str) -> bool {
    let Some(rest) = message.split("openai api error (").nth(1) else {
        return false;
    };
    rest.get(..3)
        .and_then(|status| status.parse::<u16>().ok())
        .is_some_and(|status| (500..=599).contains(&status))
}

pub(super) fn terminal_stream_retryable(error: &anyhow::Error) -> bool {
    status::terminal_stream_retryable(error)
}

pub(super) fn exhausted_request_retryable(reason: &str) -> bool {
    status::exhausted_request_retryable(reason)
}

pub(in crate::provider::openai_codex) fn tag_stream_open(error: anyhow::Error) -> anyhow::Error {
    status::tag_stream_open(error)
}

#[cfg(test)]
#[path = "classify/tests.rs"]
mod tests;
