//! Field accessors for Responses error events.

use serde_json::Value;

pub(super) fn error(event: &Value) -> Option<&Value> {
    event
        .pointer("/response/error")
        .or_else(|| event.get("error"))
}

pub(super) fn code(event: &Value) -> Option<&str> {
    error(event)?.get("code")?.as_str()
}

pub(super) fn status(event: &Value) -> Option<u64> {
    event
        .get("status")
        .or_else(|| event.get("status_code"))?
        .as_u64()
}
