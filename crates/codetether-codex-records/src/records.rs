//! Serde schemas shared by Codex discovery and transcript imports.
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
/// Canonical timestamped record decoded from a rollout line.
/// # Examples
/// ```rust
/// use codetether_codex_records::CodexJsonlRecord;
/// let raw = r#"{"timestamp":"2025-01-01T00:00:00Z","type":"event_msg","payload":{}}"#;
/// let row: CodexJsonlRecord = serde_json::from_str(raw).unwrap();
/// assert_eq!(row.kind, "event_msg");
/// ```

#[derive(Debug, Deserialize)]
pub struct CodexJsonlRecord {
    /// Timestamp associated with this record or session.
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "type")]
    /// Record category, serialized under the type key.
    pub kind: String,
    /// JSON payload retained for the category-specific decoder.
    pub payload: Value,
}
/// Identity, creation timestamp and workspace for a Codex session.
/// # Examples
/// ```rust
/// use codetether_codex_records::CodexSessionMetaPayload;
/// let raw = r#"{"id":"abc","timestamp":"2025-01-01T00:00:00Z","cwd":"/tmp"}"#;
/// let row: CodexSessionMetaPayload = serde_json::from_str(raw).unwrap();
/// assert_eq!(row.id, "abc");
/// ```

#[derive(Debug, Deserialize)]
pub struct CodexSessionMetaPayload {
    /// Codex session identifier.
    pub id: String,
    /// Timestamp associated with this record or session.
    pub timestamp: DateTime<Utc>,
    /// Working directory, possibly empty for legacy metadata.
    pub cwd: String,
}
/// Optional model selection attached to a Codex turn.
/// # Examples
/// ```rust
/// use codetether_codex_records::CodexTurnContextPayload;
/// let row: CodexTurnContextPayload = serde_json::from_str("{}").unwrap();
/// assert!(row.model.is_none());
/// ```

#[derive(Debug, Deserialize)]
pub struct CodexTurnContextPayload {
    #[serde(default)]
    /// Model selected for the turn, when available.
    pub model: Option<String>,
}
/// A title-index row with an update timestamp.
/// # Examples
/// ```rust
/// use codetether_codex_records::CodexSessionIndexEntry;
/// let raw = r#"{"id":"abc","thread_name":"task","updated_at":"2025-01-01T00:00:00Z"}"#;
/// let row: CodexSessionIndexEntry = serde_json::from_str(raw).unwrap();
/// assert_eq!(row.thread_name, "task");
/// ```

#[derive(Debug, Deserialize)]
pub struct CodexSessionIndexEntry {
    /// Codex session identifier.
    pub id: String,
    /// User-facing title recorded in the title index.
    pub thread_name: String,
    /// Time at which the index title was updated.
    pub updated_at: DateTime<Utc>,
}
