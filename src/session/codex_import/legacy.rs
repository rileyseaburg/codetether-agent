//! Legacy Codex JSONL adapter.
//!
//! Older Codex rollouts used a flat per-line schema (no `{timestamp, type,
//! payload}` envelope) and omitted `cwd` from the session meta. This module
//! transcribes those legacy records into the current [`CodexJsonlRecord`]
//! envelope so the rest of the import pipeline stays format-agnostic.
//!
//! # Legacy shapes
//!
//! | Line                | Shape                                                       |
//! |---------------------|-------------------------------------------------------------|
//! | First line (meta)   | `{id, timestamp, instructions, git}` — no `type`, no `cwd` |
//! | State marker        | `{record_type: "state"}` — ignored                          |
//! | Response items      | `{type: "message"|"reasoning"|"function_call"|...}` at top  |
//!
//! # Current shape
//!
//! `{timestamp, type: "session_meta"|"response_item"|..., payload: {...}}`
//!
//! The adapter extracts `cwd` from the first `<environment_context>` input
//! text when the legacy meta doesn't carry one (see
//! [`extract_cwd_from_env_context`]).

use super::records::{CodexJsonlRecord, CodexSessionMetaPayload};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Parses a single JSONL line into a [`CodexJsonlRecord`], accepting both the
/// modern envelope format and the legacy flat format.
///
/// # Arguments
///
/// * `raw` — One JSONL line (already trimmed of trailing newline).
/// * `session_ts` — Fallback timestamp applied to legacy response items that
///   lack a per-record timestamp. Typically the session meta timestamp
///   captured from the first line.
///
/// # Returns
///
/// * `Ok(Some(record))` — Line was parsed as a canonical envelope record.
/// * `Ok(None)` — Line was intentionally skipped (legacy `record_type: "state"`
///   markers and unrecognized legacy records).
/// * `Err(_)` — Line was not valid JSON or the modern envelope failed to
///   deserialize.
///
/// # Errors
///
/// Returns an error if `raw` is not valid JSON, or if it claims to be a modern
/// envelope (`{timestamp, type, payload}`) but fails strict deserialization.
///
/// # Examples
///
/// Modern envelope passes through unchanged:
///
/// ```rust,ignore
/// # // ignore: pub(crate) symbol, not reachable from doc tests.
/// use codetether_agent::session::codex_import::legacy::normalize_line;
///
/// let raw = r#"{"timestamp":"2025-08-28T08:45:13.563Z","type":"session_meta","payload":{"id":"abc","timestamp":"2025-08-28T08:45:13.563Z","cwd":"/tmp"}}"#;
/// let rec = normalize_line(raw, None).unwrap().unwrap();
/// assert_eq!(rec.kind, "session_meta");
/// ```
///
/// Legacy state markers are skipped:
///
/// ```rust,ignore
/// # use codetether_agent::session::codex_import::legacy::normalize_line;
/// assert!(normalize_line(r#"{"record_type":"state"}"#, None).unwrap().is_none());
/// ```
pub(crate) fn normalize_line(
    raw: &str,
    session_ts: Option<DateTime<Utc>>,
) -> Result<Option<CodexJsonlRecord>> {
    let value: Value = serde_json::from_str(raw)?;

    // Modern envelope: has top-level `type` AND `payload` AND `timestamp`.
    if value.get("type").is_some()
        && value.get("payload").is_some()
        && value.get("timestamp").is_some()
    {
        let record: CodexJsonlRecord = serde_json::from_value(value)?;
        return Ok(Some(record));
    }

    Ok(normalize_legacy(value, session_ts))
}

/// Parse a legacy first-line session meta into the canonical payload.
///
/// The legacy meta shape is `{id, timestamp, instructions, git}` at the top
/// level and lacks `cwd`. The caller must supply `cwd` (recovered elsewhere —
/// e.g. from a later `<environment_context>` message found via
/// [`extract_cwd_from_env_context`]). Pass `String::new()` when unknown; the
/// downstream parser tolerates an empty cwd.
///
/// # Returns
///
/// `None` if `value` does not contain `id` and `timestamp` as RFC-3339 strings.
///
/// # Examples
///
/// ```rust,ignore
/// # use serde_json::json;
/// # use codetether_agent::session::codex_import::legacy::legacy_session_meta;
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
pub(crate) fn legacy_session_meta(value: &Value, cwd: String) -> Option<CodexSessionMetaPayload> {
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
/// ```rust,ignore
/// # use serde_json::json;
/// # use codetether_agent::session::codex_import::legacy::is_legacy_meta;
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
pub(crate) fn is_legacy_meta(value: &Value) -> bool {
    value.get("type").is_none()
        && value.get("record_type").is_none()
        && value.get("payload").is_none()
        && value.get("id").and_then(Value::as_str).is_some()
        && value.get("timestamp").and_then(Value::as_str).is_some()
}

/// Extract the `<cwd>…</cwd>` value from a legacy `<environment_context>`
/// user message.
///
/// Legacy Codex rollouts inject a user message whose text looks like:
///
/// ```text
/// <environment_context>
///   <cwd>/home/riley/project</cwd>
///   <approval_policy>on-request</approval_policy>
///   ...
/// </environment_context>
/// ```
///
/// This function returns the trimmed inner text of the first `<cwd>` tag found
/// in any `content[].text` field of a `message` record. Returns `None` for
/// non-`message` records or when no `<cwd>` tag is present.
///
/// # Examples
///
/// ```rust,ignore
/// # use serde_json::json;
/// # use codetether_agent::session::codex_import::legacy::extract_cwd_from_env_context;
/// let v = json!({
///     "type": "message",
///     "role": "user",
///     "content": [
///         {"type": "input_text",
///          "text": "<environment_context>\n  <cwd>/home/riley/proj</cwd>\n</environment_context>"}
///     ]
/// });
/// assert_eq!(
///     extract_cwd_from_env_context(&v).as_deref(),
///     Some("/home/riley/proj")
/// );
/// ```
pub(crate) fn extract_cwd_from_env_context(value: &Value) -> Option<String> {
    if value.get("type").and_then(Value::as_str) != Some("message") {
        return None;
    }
    let content = value.get("content")?.as_array()?;
    for part in content {
        let text = part.get("text").and_then(Value::as_str)?;
        if let Some(start) = text.find("<cwd>")
            && let Some(end) = text[start + 5..].find("</cwd>")
        {
            return Some(text[start + 5..start + 5 + end].trim().to_string());
        }
    }
    None
}

/// Translate a legacy flat record into a canonical [`CodexJsonlRecord`], or
/// return `None` when the record should be skipped.
///
/// See the [module-level docs][self] for the full shape table.
fn normalize_legacy(value: Value, session_ts: Option<DateTime<Utc>>) -> Option<CodexJsonlRecord> {
    // Legacy state markers: `{record_type: "state"}` — drop.
    if value.get("record_type").is_some() {
        return None;
    }

    // Legacy session meta (first line): `{id, timestamp, instructions, git}`.
    if is_legacy_meta(&value) {
        let meta = legacy_session_meta(&value, String::new())?;
        let payload = serde_json::json!({
            "id": meta.id,
            "timestamp": meta.timestamp.to_rfc3339(),
            "cwd": meta.cwd,
        });
        return Some(CodexJsonlRecord {
            timestamp: meta.timestamp,
            kind: "session_meta".to_string(),
            payload,
        });
    }

    // Legacy response items: `{type: "message"|"reasoning"|"function_call"|...}`.
    if let Some(kind) = value.get("type").and_then(Value::as_str)
        && matches!(
            kind,
            "message" | "reasoning" | "function_call" | "function_call_output"
        )
    {
        let timestamp = session_ts.unwrap_or_else(Utc::now);
        return Some(CodexJsonlRecord {
            timestamp,
            kind: "response_item".to_string(),
            payload: value,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_modern_envelope() {
        let raw = r#"{"timestamp":"2025-08-28T08:45:13.563Z","type":"session_meta","payload":{"id":"abc","timestamp":"2025-08-28T08:45:13.563Z","cwd":"/tmp"}}"#;
        let rec = normalize_line(raw, None).unwrap().unwrap();
        assert_eq!(rec.kind, "session_meta");
    }

    #[test]
    fn normalizes_legacy_meta() {
        let raw = r#"{"id":"ce59ef14","timestamp":"2025-08-28T08:45:13.563Z","instructions":"hi","git":{}}"#;
        let rec = normalize_line(raw, None).unwrap().unwrap();
        assert_eq!(rec.kind, "session_meta");
        let meta: CodexSessionMetaPayload = serde_json::from_value(rec.payload).unwrap();
        assert_eq!(meta.id, "ce59ef14");
        assert_eq!(meta.cwd, "");
    }

    #[test]
    fn skips_legacy_state_marker() {
        let raw = r#"{"record_type":"state"}"#;
        assert!(normalize_line(raw, None).unwrap().is_none());
    }

    #[test]
    fn normalizes_legacy_response_item() {
        let raw = r#"{"type":"message","id":null,"role":"user","content":[{"type":"input_text","text":"hi"}]}"#;
        let ts: DateTime<Utc> = "2025-08-28T08:45:13.563Z".parse().unwrap();
        let rec = normalize_line(raw, Some(ts)).unwrap().unwrap();
        assert_eq!(rec.kind, "response_item");
        assert_eq!(rec.payload.get("type").unwrap(), "message");
    }

    #[test]
    fn extracts_cwd_from_env_context() {
        let raw = r#"{"type":"message","role":"user","content":[{"type":"input_text","text":"<environment_context>\n  <cwd>/home/riley/spotlessbinco</cwd>\n</environment_context>"}]}"#;
        let value: Value = serde_json::from_str(raw).unwrap();
        assert_eq!(
            extract_cwd_from_env_context(&value).as_deref(),
            Some("/home/riley/spotlessbinco")
        );
    }
}
