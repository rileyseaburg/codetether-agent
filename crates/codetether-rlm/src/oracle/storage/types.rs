//! Serializable outcome types for oracle trace persistence.
//!
//! These structs are returned by the storage layer and surfaced
//! to CLI callers and the TUI so users can see whether a trace
//! was uploaded, kept locally, or failed.
//!
//! # Examples
//!
//! ```ignore
//! let result = storage.persist_result(&oracle_result).await?;
//! println!("verdict={} uploaded={}", result.verdict, result.uploaded);
//! ```

use serde::{Deserialize, Serialize};

/// Captures the outcome of writing a single oracle trace to
/// the local spool and (optionally) uploading it to remote
/// storage.
///
/// Always includes the local spool path. When a remote backend
/// is configured and the upload succeeds, `uploaded` is `true`
/// and `remote_key` / `remote_url` are populated.
///
/// # Examples
///
/// ```ignore
/// let persist = storage.persist_record(&record).await?;
/// if persist.uploaded {
///     println!("remote: {}", persist.remote_url.unwrap());
/// } else {
///     println!("local only: {}", persist.spooled_path);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleTracePersistResult {
    /// Classification label (e.g. "golden", "failed").
    pub verdict: String,
    /// Absolute path to the local spool `.jsonl` file.
    pub spooled_path: String,
    /// Whether the record reached remote storage.
    pub uploaded: bool,
    /// S3/MinIO object key, if uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_key: Option<String>,
    /// Full URL to the uploaded object, if uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    /// Number of `.jsonl` files still in the spool.
    pub pending_count: usize,
    /// Human-readable note when something went wrong.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}
