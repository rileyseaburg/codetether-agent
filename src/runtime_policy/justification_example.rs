//! Ready-to-retry argument example for a missing `ask`-mode justification.
//!
//! The model's own arguments are echoed back with only a `justification`
//! slot added, so the retry differs from the blocked call in one field and the
//! approval resource stays identical. Long values (patch bodies, file content)
//! are elided to keep the error small.

use serde_json::{Map, Value, json};

/// Internal or transport-only keys that must never be echoed back.
const HIDDEN: &[&str] = &["approval_id", "_tool_call_id", "__ct_parent_workspace"];

/// Strings longer than this are replaced by [`UNCHANGED`].
const MAX_ECHO_CHARS: usize = 160;

/// Marker telling the model to resend the original value verbatim.
pub(super) const UNCHANGED: &str = "<unchanged: resend the original value>";

/// Placeholder shown in the `justification` slot of the example.
pub(super) const PLACEHOLDER: &str = "<why this action is needed for the current request>";

/// Original arguments plus a `justification` placeholder.
///
/// Non-object arguments fall back to a minimal `{ "justification": ... }`.
pub(super) fn retry(args: &Value) -> Value {
    let mut example = Map::new();
    for (key, value) in args.as_object().into_iter().flatten() {
        if !HIDDEN.contains(&key.as_str()) {
            example.insert(key.clone(), elide(value));
        }
    }
    example.insert(super::justification::FIELD.into(), json!(PLACEHOLDER));
    Value::Object(example)
}

fn elide(value: &Value) -> Value {
    let long = match value {
        Value::String(text) => text.chars().count() > MAX_ECHO_CHARS,
        Value::Array(_) | Value::Object(_) => value.to_string().len() > MAX_ECHO_CHARS,
        Value::Null | Value::Bool(_) | Value::Number(_) => false,
    };
    if long {
        json!(UNCHANGED)
    } else {
        value.clone()
    }
}

#[cfg(test)]
#[path = "justification_example_tests.rs"]
mod tests;
