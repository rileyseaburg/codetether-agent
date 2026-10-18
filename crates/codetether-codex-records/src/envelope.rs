//! Parse an envelope or delegate flat records to the legacy adapter.
use crate::{CodexJsonlRecord, legacy::normalize_legacy};
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
/// ```rust
/// use codetether_codex_records::normalize_line;
///
/// let raw = r#"{"timestamp":"2025-08-28T08:45:13.563Z","type":"session_meta","payload":{"id":"abc","timestamp":"2025-08-28T08:45:13.563Z","cwd":"/tmp"}}"#;
/// let rec = normalize_line(raw, None).unwrap().unwrap();
/// assert_eq!(rec.kind, "session_meta");
/// ```
///
/// Legacy state markers are skipped:
///
/// ```rust
/// # use codetether_codex_records::normalize_line;
/// assert!(normalize_line(r#"{"record_type":"state"}"#, None).unwrap().is_none());
/// ```
pub fn normalize_line(
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
