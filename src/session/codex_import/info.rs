use chrono::{DateTime, Utc};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CodexSessionInfo {
    pub id: String,
    pub path: PathBuf,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
    pub agent: String,
    pub directory: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CodexImportReport {
    pub imported: usize,
    pub skipped: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PersistOutcome {
    Saved,
    Unchanged,
}
