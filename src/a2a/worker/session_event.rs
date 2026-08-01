//! Structured session events streamed alongside task output.
//!
//! Tool activity used to reach the server only as pre-joined text such as
//! `[tool:bash:ok] ...`, truncated to 500 bytes. That flattening destroyed the
//! tool name, arguments, and status before publish, so the Forgejo transcript
//! could only ever classify every line as `agent.message` and never render a
//! tool card.
//!
//! These helpers build the typed payloads that travel next to the human-readable
//! text on the existing signed output request, so no callback signature changes.

use serde_json::{Map, Value};

/// Maximum bytes of tool output carried in one event payload.
///
/// Generous enough for a real `bash` result or diff while still bounding the
/// events table. Truncation is always recorded explicitly rather than silently
/// clipping, so a reader can tell a short result from a cut-off one.
pub const MAX_OUTPUT_BYTES: usize = 8192;

/// Maximum bytes of a single serialized tool argument value.
pub const MAX_ARG_BYTES: usize = 2048;

/// Argument names whose values must never be published.
///
/// Structured arguments are a new exposure surface: a flattened 500-byte string
/// often hid a credential that a full `bash` command or `write` body would now
/// reveal. Over-redacting is preferred to leaking a token onto an issue page.
const SENSITIVE_KEYS: &[&str] = &[
    "token",
    "secret",
    "password",
    "passwd",
    "authorization",
    "api_key",
    "apikey",
    "access_key",
    "private_key",
    "credential",
    "credentials",
    "session_token",
    "bearer",
];

/// Returns true when an argument name looks like it carries a secret.
///
/// # Examples
///
/// ```ignore
/// assert!(is_sensitive_key("GITHUB_TOKEN"));
/// assert!(!is_sensitive_key("path"));
/// ```
pub fn is_sensitive_key(key: &str) -> bool {
    let lowered = key.to_ascii_lowercase();
    SENSITIVE_KEYS.iter().any(|needle| lowered.contains(needle))
}

/// Returns `text` bounded to `limit` bytes on a character boundary.
fn clamp(text: &str, limit: usize) -> (String, bool) {
    if text.len() <= limit {
        return (text.to_string(), false);
    }
    (
        crate::util::truncate_bytes_safe(text, limit).to_string(),
        true,
    )
}

/// Redacts secret-bearing values and bounds oversized ones.
///
/// # Examples
///
/// ```ignore
/// let safe = redact_arguments(&serde_json::json!({"token": "abc"}));
/// assert_eq!(safe["token"], serde_json::json!("[redacted]"));
/// ```
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

/// Builds a `tool.call` payload for one invocation.
///
/// # Examples
///
/// ```ignore
/// let event = tool_call_event("bash", &serde_json::json!({"command": "ls"}));
/// assert_eq!(event["type"], serde_json::json!("tool.call"));
/// ```
pub fn tool_call_event(tool_name: &str, arguments: &Value) -> Value {
    serde_json::json!({
        "type": "tool.call",
        "payload": {
            "name": tool_name,
            "tool": tool_name,
            "status": "started",
            "arguments": redact_arguments(arguments),
        },
    })
}

/// Builds a `tool.result` payload for one completed invocation.
///
/// # Examples
///
/// ```ignore
/// let event = tool_result_event("bash", true, "ok", None);
/// assert_eq!(event["payload"]["status"], serde_json::json!("ok"));
/// ```
pub fn tool_result_event(
    tool_name: &str,
    success: bool,
    output: &str,
    duration_ms: Option<u128>,
) -> Value {
    let (bounded, truncated) = clamp(output, MAX_OUTPUT_BYTES);
    let mut payload = serde_json::json!({
        "name": tool_name,
        "tool": tool_name,
        "status": if success { "ok" } else { "err" },
        "success": success,
        "output": bounded,
        "output_truncated": truncated,
        "output_bytes": output.len(),
    });
    if let Some(duration) = duration_ms
        && let Some(fields) = payload.as_object_mut()
    {
        fields.insert("duration_ms".into(), Value::from(duration as u64));
    }
    serde_json::json!({ "type": "tool.result", "payload": payload })
}

/// Builds an `agent.reasoning` payload.
///
/// Reasoning was previously never captured, so the transcript could show what
/// the agent did but never why.
///
/// # Examples
///
/// ```ignore
/// let event = reasoning_event("Checking the failing test first.");
/// assert_eq!(event["type"], serde_json::json!("agent.reasoning"));
/// ```
pub fn reasoning_event(text: &str) -> Value {
    let (bounded, truncated) = clamp(text, MAX_OUTPUT_BYTES);
    serde_json::json!({
        "type": "agent.reasoning",
        "payload": {
            "role": "assistant",
            "content": bounded,
            "content_truncated": truncated,
        },
    })
}

#[cfg(test)]
#[path = "session_event_tests.rs"]
mod tests;
