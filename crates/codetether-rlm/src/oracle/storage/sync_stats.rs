//! Aggregate counters produced by a bulk spool-to-remote sync.
//!
//! Returned by [`super::manager::OracleTraceStorage::sync_pending`]
//! so callers can log progress or alert on failures.
//!
//! # Examples
//!
//! ```ignore
//! let stats = storage.sync_pending().await?;
//! println!("uploaded={} failed={}", stats.uploaded, stats.failed);
//! ```

use serde::{Deserialize, Serialize};

/// Counters from a single sync pass over the local spool.
///
/// `retained` includes both intentionally-kept and
/// failed-to-upload files. `pending_after` reflects the spool
/// state after the pass completes.
///
/// # Examples
///
/// ```ignore
/// let stats = OracleTraceSyncStats::default();
/// assert_eq!(stats.uploaded, 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OracleTraceSyncStats {
    /// Files successfully uploaded and removed from spool.
    pub uploaded: usize,
    /// Files that remain in the spool after this pass.
    pub retained: usize,
    /// Files that could not be uploaded or deleted.
    pub failed: usize,
    /// Total pending spool files after the pass.
    pub pending_after: usize,
}
