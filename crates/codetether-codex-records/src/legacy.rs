//! Convert recognized flat records into canonical envelopes.
use crate::{CodexJsonlRecord, is_legacy_meta, legacy_session_meta};
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Translate a legacy flat record into a canonical [`CodexJsonlRecord`], or
/// return `None` when the record should be skipped.
///
/// Metadata becomes a session_meta record; response items retain their payload.
pub(crate) fn normalize_legacy(
    value: Value,
    session_ts: Option<DateTime<Utc>>,
) -> Option<CodexJsonlRecord> {
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
