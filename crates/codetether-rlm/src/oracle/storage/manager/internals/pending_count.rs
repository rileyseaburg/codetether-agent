//! Focused storage helper.
use crate::oracle::storage::manager::OracleTraceStorage;
use anyhow::Result;

impl OracleTraceStorage {
    pub(in crate::oracle::storage) async fn pending_count(&self) -> Result<usize> {
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
}
