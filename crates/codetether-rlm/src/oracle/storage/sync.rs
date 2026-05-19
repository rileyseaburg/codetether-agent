//! Bulk sync of pending spool files to remote storage.
//!
//! Iterates every `.jsonl` file in the spool directory, attempts
//! to upload each one, and removes the local copy on success.
//! Invalid or upload-failed files are retained for the next pass.
//!
//! # Examples
//!
//! ```ignore
//! let stats = storage.sync_pending().await?;
//! println!("uploaded={} failed={}", stats.uploaded, stats.failed);
//! ```

use super::manager::OracleTraceStorage;
use super::sync_stats::OracleTraceSyncStats;
use crate::oracle::record::OracleTraceRecord;
use anyhow::{Context, Result};

impl OracleTraceStorage {
    /// Upload all pending spool records to the remote backend.
    ///
    /// Files that fail to parse or upload remain on disk so a
    /// later invocation can retry. Returns aggregate counters.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let stats = storage.sync_pending().await?;
    /// assert_eq!(stats.pending_after, 0);
    /// ```
    pub async fn sync_pending(&self) -> Result<OracleTraceSyncStats> {
        tokio::fs::create_dir_all(&self.spool_dir)
            .await
            .with_context(|| format!("Create spool dir {}", self.spool_dir.display()))?;

        let Some(remote) = &self.remote else {
            let n = self.pending_count().await?;
            return Ok(OracleTraceSyncStats {
                retained: n,
                pending_after: n,
                ..Default::default()
            });
        };

        let mut stats = OracleTraceSyncStats::default();
        let mut dir = tokio::fs::read_dir(&self.spool_dir).await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
                continue;
            }
            let data = tokio::fs::read_to_string(&path).await?;
            let record: OracleTraceRecord = match serde_json::from_str(&data) {
                Ok(r) => r,
                Err(e) => {
                    stats.failed += 1;
                    stats.retained += 1;
                    tracing::warn!(path = %path.display(), error = %e, "Invalid spool file");
                    continue;
                }
            };
            match remote.upload_record(&record).await {
                Ok(_) => {
                    stats.uploaded += 1;
                    if let Err(e) = tokio::fs::remove_file(&path).await {
                        stats.failed += 1;
                        stats.retained += 1;
                        tracing::warn!(path = %path.display(), error = %e, "Cleanup failed");
                    }
                }
                Err(e) => {
                    stats.failed += 1;
                    stats.retained += 1;
                    tracing::warn!(path = %path.display(), error = %e, "Sync failed");
                }
            }
        }
        stats.pending_after = self.pending_count().await?;
        Ok(stats)
    }
}
