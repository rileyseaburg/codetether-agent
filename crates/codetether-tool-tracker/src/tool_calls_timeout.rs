//! Reading a tool call's requested runtime from its arguments.

use std::time::Duration;

/// `timeout` (seconds), `timeout_secs`, or `timeout_ms` from a tool's arguments.
pub(super) fn declared_timeout(arguments: &serde_json::Value) -> Option<Duration> {
    let secs = arguments
        .get("timeout")
        .or_else(|| arguments.get("timeout_secs"))
        .and_then(serde_json::Value::as_u64)
        .map(Duration::from_secs);
    secs.or_else(|| {
        arguments
            .get("timeout_ms")
            .and_then(serde_json::Value::as_u64)
            .map(Duration::from_millis)
    })
}
