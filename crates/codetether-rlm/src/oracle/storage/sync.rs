//! Iterate pending spool files and aggregate sync outcomes.

use super::{manager::OracleTraceStorage, sync_stats::OracleTraceSyncStats};
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
    ///
    /// # Errors
    /// Returns an error if a directory, file, serialization, or configured storage operation fails.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_rlm::{OracleTraceStorage};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let storage = OracleTraceStorage::from_env_or_vault().await;
    /// let stats = storage.sync_pending().await.unwrap();
    /// assert!(stats.pending_after >= stats.retained);
    /// # });
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
            super::sync_record::sync_record(remote.as_ref(), &path, &mut stats).await?;
        }
        stats.pending_after = self.pending_count().await?;
        Ok(stats)
    }
}
