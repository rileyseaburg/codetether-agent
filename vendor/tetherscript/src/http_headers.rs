//! HTTP header helpers for server-side request/response handling.

use crate::value::Value;

/// Return true when request headers explicitly ask to close the socket.
pub(crate) fn wants_close(request: &Value) -> bool {
    header_value(request, "connection")
        .map(|value| has_token(&value, "close"))
        .unwrap_or(false)
}

fn header_value(request: &Value, name: &str) -> Option<String> {
    let map = match request {
        Value::Map(map) => map.borrow(),
        _ => return None,
    };
    let headers = match map.get("headers") {
        Some(Value::Map(h)) => h.borrow(),
        _ => return None,
    };
    match headers.get(name) {
        Some(Value::Str(s)) => Some((**s).clone()),
        _ => None,
    }
}

fn has_token(value: &str, token: &str) -> bool {
    value
        .split(',')
        .any(|part| part.trim().eq_ignore_ascii_case(token))
}
