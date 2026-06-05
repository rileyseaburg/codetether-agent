use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Lightweight metadata shown in session lists.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::SessionSummary;
///
/// let encoded = r#"{
///   "id":"s1","title":null,"created_at":"2026-01-01T00:00:00Z",
///   "updated_at":"2026-01-01T00:00:00Z","message_count":0,
///   "agent":"default","directory":null
/// }"#;
/// let summary: SessionSummary = serde_json::from_str(encoded).unwrap();
/// assert_eq!(summary.id, "s1");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    /// UUID identifying this session on disk.
    pub id: String,
    /// Optional human-readable title.
    pub title: Option<String>,
    /// When the session was created.
    pub created_at: DateTime<Utc>,
    /// When the session was last updated.
    pub updated_at: DateTime<Utc>,
    /// Number of persisted chat messages.
    pub message_count: usize,
    /// Agent persona name.
    pub agent: String,
    /// Workspace directory this session belongs to.
    #[serde(default)]
    pub directory: Option<PathBuf>,
}
