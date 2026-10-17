//! Serializable record of a locally written and archived event file.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Local path, remote URL, session, byte offsets, and archival timestamp.
///
/// # Examples
/// ```
/// use codetether_event_stream::s3_sink::ArchivedEventFile;
/// let file = ArchivedEventFile { local_path: "events.jsonl".into(),
///     s3_url: "s3://logs/events.jsonl".into(), session_id: "run".into(),
///     start_offset: 0, end_offset: 10, archived_at: chrono::Utc::now() };
/// assert_eq!(file.end_offset, 10);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ArchivedEventFile {
    /// Local path where the file was written
    pub local_path: PathBuf,
    /// S3 URL where the file was archived
    pub s3_url: String,
    /// Session ID
    pub session_id: String,
    /// Byte range of events in this file
    pub start_offset: u64,
    pub end_offset: u64,
    /// When the file was archived
    pub archived_at: chrono::DateTime<chrono::Utc>,
}
