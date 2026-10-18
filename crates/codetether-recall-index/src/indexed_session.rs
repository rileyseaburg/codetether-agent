//! Persisted recall projection for one session.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::document::RecallDocument;

/// Supported persisted recall-projection format.
pub const SCHEMA_VERSION: u8 = 1;

/// Compact, searchable projection of a persisted session.
/// # Examples
/// ```
/// use codetether_recall_index::{IndexedSession, SCHEMA_VERSION};
/// let session = IndexedSession { schema_version: SCHEMA_VERSION,
///     session_id: "id".into(), title: None, workspace: "/tmp".into(),
///     updated_at: chrono::Utc::now(), message_count: 0, documents: vec![] };
/// assert!(session.is_current_schema());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedSession {
    /// Sidecar schema version.
    pub schema_version: u8,
    /// Source session UUID.
    pub session_id: String,
    /// Optional user-facing session title.
    pub title: Option<String>,
    /// Canonical workspace associated with the session.
    pub workspace: PathBuf,
    /// Source snapshot modification time.
    pub updated_at: DateTime<Utc>,
    /// Source transcript size used for stale-write rejection.
    pub message_count: usize,
    /// Searchable evidence ranges.
    pub documents: Vec<RecallDocument>,
}

impl IndexedSession {
    /// Whether this projection uses the supported serialized schema.
    pub fn is_current_schema(&self) -> bool {
        self.schema_version == SCHEMA_VERSION
    }
}
