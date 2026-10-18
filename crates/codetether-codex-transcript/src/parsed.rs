//! Provider-level transcript data ready for native session construction.
use chrono::{DateTime, Utc};
use codetether_provider_core::{Message, Usage};
use std::path::PathBuf;
/// Bounded transcript and metadata, independent of application storage.
/// # Examples
/// ```no_run
/// let parsed = codetether_codex_transcript::parse_codex_session_from_path(
///     std::path::Path::new("rollout.jsonl"), Some("Imported task"))?;
/// assert_eq!(parsed.title.as_deref(), Some("Imported task"));
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct ParsedSession {
    /// Session identifier from archive metadata.
    pub id: String,
    /// Overridden or first-user-derived title.
    pub title: Option<String>,
    /// Original creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Maximum timestamp encountered in the archive.
    pub updated_at: DateTime<Utc>,
    /// Most recent converted messages in source order.
    pub messages: Vec<Message>,
    /// Latest token-count event, or zero usage.
    pub usage: Usage,
    /// Canonical workspace when available, otherwise the original path.
    pub directory: Option<PathBuf>,
    /// Last nonempty model from a turn context.
    pub model: Option<String>,
}
