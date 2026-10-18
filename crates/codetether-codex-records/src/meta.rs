//! Recognition and projection of legacy session metadata.
use crate::CodexSessionMetaPayload;
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Parse a legacy first-line session meta into the canonical payload.
///
/// The legacy meta shape is `{id, timestamp, instructions, git}` at the top
/// level and lacks `cwd`. The caller must supply `cwd` (recovered elsewhere —
/// e.g. from a later `<environment_context>` message found via
/// [`crate::extract_cwd_from_env_context`]). Pass `String::new()` when unknown; the
/// downstream parser tolerates an empty cwd.
///
/// # Returns
///
/// `None` if `value` does not contain `id` and `timestamp` as RFC-3339 strings.
///
/// # Examples
///
/// ```rust
/// # use serde_json::json;
/// # use codetether_codex_records::legacy_session_meta;
/// let v = json!({
///     "id": "ce59ef14",
///     "timestamp": "2025-08-28T08:45:13.563Z",
///     "instructions": "...",
///     "git": {}
/// });
/// let meta = legacy_session_meta(&v, "/home/riley/project".into()).unwrap();
/// assert_eq!(meta.id, "ce59ef14");
/// assert_eq!(meta.cwd, "/home/riley/project");
/// ```
/// # Arguments
/// * `value` — Legacy metadata object.
/// * `cwd` — Recovered working directory, or an empty string if unknown.
pub fn legacy_session_meta(value: &Value, cwd: String) -> Option<CodexSessionMetaPayload> {
    let id = value.get("id")?.as_str()?.to_string();
    let ts_str = value.get("timestamp")?.as_str()?;
    let timestamp = DateTime::parse_from_rfc3339(ts_str)
        .ok()?
        .with_timezone(&Utc);
    Some(CodexSessionMetaPayload { id, timestamp, cwd })
}

/// Heuristic check for whether a raw JSON value looks like a *legacy*
/// session-meta first line (as opposed to a modern envelope or a response item).
///
/// Returns `true` when the value has string fields `id` and `timestamp` at the
/// top level and has neither `type`, `record_type`, nor `payload`.
///
/// # Examples
///
/// ```rust
/// # use serde_json::json;
/// # use codetether_codex_records::is_legacy_meta;
/// assert!(is_legacy_meta(&json!({
///     "id": "abc", "timestamp": "2025-08-28T08:45:13.563Z", "git": {}
/// })));
/// // Modern envelope — not legacy.
/// assert!(!is_legacy_meta(&json!({
///     "timestamp": "2025-08-28T08:45:13.563Z",
///     "type": "session_meta",
///     "payload": {}
/// })));
/// ```
/// # Arguments
/// * `value` — JSON value to classify.
/// # Returns
/// Whether the value matches the legacy session metadata shape.
pub fn is_legacy_meta(value: &Value) -> bool {
    value.get("type").is_none()
        && value.get("record_type").is_none()
        && value.get("payload").is_none()
        && value.get("id").and_then(Value::as_str).is_some()
        && value.get("timestamp").and_then(Value::as_str).is_some()
}
