//! Public persist methods for [`OracleTraceStorage`].
//!
//! Handles spool writes and delegates upload attempts to
//! internal helpers in [`super::internals`].
//!
//! # Examples
//!
//! ```ignore
//! let out = storage.persist_record(&record).await?;
//! assert!(!out.spooled_path.is_empty());
//! ```

use super::super::spool_io::{spool_filename, write_json_atomic};
use super::super::types::OracleTracePersistResult;
use super::OracleTraceStorage;
use crate::rlm::oracle::record::OracleTraceRecord;
use crate::rlm::oracle::trace_types::OracleResult;
use anyhow::{Context, Result};
use chrono::Utc;

impl OracleTraceStorage {
    /// Convert an [`OracleResult`] to a record and persist it.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let out = storage.persist_result(&oracle_result).await?;
    /// ```
    pub async fn persist_result(&self, result: &OracleResult) -> Result<OracleTracePersistResult> {
        self.persist_record(&result.to_record()).await
    }

    /// Write `record` to local spool and attempt remote upload.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let out = storage.persist_record(&record).await?;
    /// ```
    pub async fn persist_record(
        &self,
        record: &OracleTraceRecord,
    ) -> Result<OracleTracePersistResult> {
        tokio::fs::create_dir_all(&self.spool_dir)
            .await
            .with_context(|| format!("Create spool dir {}", self.spool_dir.display()))?;
        self.warn_if_spool_large().await;

        let path = self
            .spool_dir
            .join(spool_filename(record, Utc::now().timestamp_millis()));
        write_json_atomic(&path, record).await?;

        let (uploaded, remote_key, remote_url, warning) = self.try_upload(record, &path).await;
        let pending_count = self.pending_count().await?;
        Ok(OracleTracePersistResult {
            verdict: record.verdict.clone(),
            spooled_path: path.display().to_string(),
            uploaded,
            remote_key,
            remote_url,
            pending_count,
            warning,
        })
    }
}
