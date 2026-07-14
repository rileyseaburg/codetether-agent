//! Persisted recall projection for one session.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::document::RecallDocument;

pub(crate) const SCHEMA_VERSION: u8 = 1;

/// Compact, searchable projection of a persisted session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IndexedSession {
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
    pub(crate) fn is_current_schema(&self) -> bool {
        self.schema_version == SCHEMA_VERSION
    }
}
