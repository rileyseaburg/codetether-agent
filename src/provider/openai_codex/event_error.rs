//! Codex Responses error-event classification.

use serde_json::Value;

#[path = "event_error/fields.rs"]
mod fields;

const PERMANENT_CODES: &[&str] = &[
    "context_length_exceeded",
    "insufficient_quota",
    "usage_not_included",
    "cyber_policy",
    "invalid_prompt",
    "bio_policy",
    "slow_down",
];
const CONNECTION_LIMIT: &str = "websocket_connection_limit_reached";

pub(super) fn is_terminal(event: &Value) -> bool {
    fields::code(event) == Some(CONNECTION_LIMIT) || fields::status(event).is_some()
}

pub(super) fn format(event: &Value, fallback: &str) -> String {
    let message = fields::error(event)
        .and_then(|value| value.get("message"))
        .or_else(|| event.get("message"))
        .and_then(Value::as_str)
        .unwrap_or(fallback);
    let prefix = if retryable(event) {
        "codex-retryable: "
    } else {
        "codex-permanent: "
    };
    format!("{prefix}{message}")
}

fn retryable(event: &Value) -> bool {
    if fields::code(event) == Some(CONNECTION_LIMIT) {
        return true;
    }
    if fields::code(event).is_some_and(|code| PERMANENT_CODES.contains(&code)) {
        return false;
    }
    !matches!(fields::status(event), Some(400 | 429))
}