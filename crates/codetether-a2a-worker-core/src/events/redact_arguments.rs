//! Structured worker event redact arguments.
use super::*;

/// Redacts secret-bearing values and bounds oversized ones.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::events::redact_arguments;
/// let safe = redact_arguments(&serde_json::json!({"token": "abc"}));
/// assert_eq!(safe["token"], serde_json::json!("[redacted]"));
/// ```
/// # Arguments
/// * `input` - Argument value to recursively redact and bound.
/// # Returns
/// A redacted and bounded copy of the supplied value.
pub fn redact_arguments(input: &Value) -> Value {
    match input {
        Value::Object(fields) => {
            let mut safe = Map::new();
            for (key, value) in fields {
                if is_sensitive_key(key) {
                    safe.insert(key.clone(), Value::from("[redacted]"));
                    continue;
                }
                safe.insert(key.clone(), redact_arguments(value));
            }
            Value::Object(safe)
        }
        Value::Array(items) => Value::Array(items.iter().map(redact_arguments).collect()),
        Value::String(text) => {
            let (bounded, truncated) = clamp(text, MAX_ARG_BYTES);
            if truncated {
                Value::from(format!("{bounded}… [truncated]"))
            } else {
                Value::String(text.clone())
            }
        }
        other => other.clone(),
    }
}
