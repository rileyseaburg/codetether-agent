//! Decide whether a reasoning item contributes to the summary message count.
use serde_json::Value;

pub(crate) fn count_reasoning(payload: &Value) -> bool {
    payload
        .get("content")
        .and_then(Value::as_str)
        .is_some_and(|text| !text.trim().is_empty())
        || payload
            .get("summary")
            .and_then(Value::as_array)
            .is_some_and(|items| !items.is_empty())
}
