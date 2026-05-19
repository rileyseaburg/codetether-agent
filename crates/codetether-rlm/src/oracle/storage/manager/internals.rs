//! Internal helpers for upload attempts, pending counts, and
//! spool-size warnings used by [`OracleTraceStorage`].
//!
//! These methods are implementation details that keep the
//! public `persist_*` surface thin.
//!
//! # Examples
//!
//! ```ignore
//! let n = storage.pending_count().await?;
//! ```

use super::super::usage::spool_usage_bytes;
use super::OracleTraceStorage;
use crate::oracle::record::OracleTraceRecord;
use anyhow::Result;

impl OracleTraceStorage {
    pub(in super::super) async fn try_upload(
        &self,
        record: &OracleTraceRecord,
        spool_path: &std::path::Path,
    ) -> (bool, Option<String>, Option<String>, Option<String>) {
        let Some(remote) = &self.remote else {
            return (false, None, None, Some("Remote not configured".into()));
        };
        match remote.upload_record(record).await {
            Ok((key, url)) => {
                let w = tokio::fs::remove_file(spool_path)
                    .await
                    .err()
                    .map(|e| format!("Uploaded but cleanup failed: {e}"));
                (true, Some(key), Some(url), w)
            }
            Err(e) => (false, None, None, Some(format!("Upload failed: {e}"))),
        }
    }

    pub(in super::super) async fn pending_count(&self) -> Result<usize> {
        if !self.spool_dir.exists() {
            return Ok(0);
        }
        let mut count = 0usize;
        let mut dir = tokio::fs::read_dir(&self.spool_dir).await?;
        while let Some(entry) = dir.next_entry().await? {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("jsonl") {
                count += 1;
            }
        }
        Ok(count)
    }

    pub(in super::super) async fn warn_if_spool_large(&self) {
        match spool_usage_bytes(&self.spool_dir).await {
            Ok(b) if b > self.max_spool_bytes => {
                tracing::warn!(
                    usage_bytes = b,
                    max = self.max_spool_bytes,
                    "Spool over limit"
                );
            }
            Ok(_) => {}
            Err(e) => {
                tracing::debug!(error = %e, "Spool usage check failed");
            }
        }
    }
}
