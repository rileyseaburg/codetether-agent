//! Server-verified author binding for reusable Forgejo sessions.

use serde_json::{Map, Value};

const PROTOCOL: &str = "codetether.forgejo-author.v1";

pub(super) fn verified(metadata: &Map<String, Value>) -> bool {
    metadata.get("protocol").and_then(Value::as_str) == Some(PROTOCOL)
        && metadata
            .get("server_author_binding_verified")
            .and_then(Value::as_bool)
            == Some(true)
}

pub(super) fn resume_session_id(metadata: &Map<String, Value>) -> Option<String> {
    let value = metadata
        .get("resume_session_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    if metadata.get("protocol").and_then(Value::as_str) == Some(PROTOCOL) {
        return verified(metadata).then_some(value).flatten();
    }
    value
}

pub(super) fn preserve_workspace(
    metadata: &Map<String, Value>,
    session_id: &Option<String>,
) -> bool {
    verified(metadata)
        && session_id.is_some()
        && metadata.get("provenance_verified").and_then(Value::as_bool) == Some(true)
        && metadata
            .get("preserve_session_workspace")
            .and_then(Value::as_bool)
            == Some(true)
}

#[cfg(test)]
#[path = "task_author_binding_tests.rs"]
mod tests;
