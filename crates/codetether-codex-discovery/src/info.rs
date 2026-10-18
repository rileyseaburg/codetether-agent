//! Metadata summary for an archived Codex session.
use chrono::{DateTime, Utc};
use std::path::PathBuf;

/// Identity and lightweight metadata from an archived Codex session.
/// # Examples
/// ```no_run
/// let rows = codetether_codex_discovery::discover_codex_sessions_in(None, std::path::Path::new(".")).unwrap();
/// assert!(rows.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct CodexSessionInfo {
    /// Session identifier.
    pub id: String,
    /// Path to the source rollout.
    pub path: PathBuf,
    /// Indexed or derived session title.
    pub title: Option<String>,
    /// Original session creation time.
    pub created_at: DateTime<Utc>,
    /// Newest record timestamp.
    pub updated_at: DateTime<Utc>,
    /// Count of visible messages and tool records.
    pub message_count: usize,
    /// Native agent name assigned to imports.
    pub agent: String,
    /// Recovered workspace directory, if available.
    pub directory: Option<PathBuf>,
}
