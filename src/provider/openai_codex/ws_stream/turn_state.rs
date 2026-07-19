//! Capture sticky-routing metadata from Responses WebSocket events.

use serde_json::Value;
use tokio_tungstenite::tungstenite::http::{HeaderValue, Request};

use super::super::{TurnStateStore, X_CODEX_TURN_STATE_HEADER};

pub(super) fn capture(states: &TurnStateStore, session_id: &str, event: &Value) {
    if event.get("type").and_then(Value::as_str) != Some("response.metadata") {
        return;
    }
    let value = event
        .get("headers")
        .and_then(Value::as_object)
        .and_then(|headers| {
            headers.iter().find_map(|(name, value)| {
                name.eq_ignore_ascii_case(X_CODEX_TURN_STATE_HEADER)
                    .then(|| value_string(value))
                    .flatten()
            })
        });
    if let Some(value) = value {
        states.capture(session_id, value);
    }
}

pub(in super::super) fn replay(
    states: &TurnStateStore,
    session_id: &str,
    request: &mut Request<()>,
) {
    let state = states.current(session_id);
    if let Some(value) = state.get()
        && let Ok(value) = HeaderValue::from_str(value)
    {
        request
            .headers_mut()
            .insert(X_CODEX_TURN_STATE_HEADER, value);
    }
}

fn value_string(value: &Value) -> Option<&str> {
    match value {
        Value::String(value) => Some(value),
        Value::Array(values) => values.first().and_then(value_string),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::Object(_) => None,
    }
}

#[cfg(test)]
#[path = "turn_state/tests.rs"]
mod tests;
