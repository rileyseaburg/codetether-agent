//! Decode one spool record and retain malformed records.

use super::{remote::OracleRemote, sync_stats::OracleTraceSyncStats};
use crate::oracle::record::OracleTraceRecord;
use anyhow::Result;
use std::path::Path;
pub(super) async fn sync_record(
    remote: &dyn OracleRemote,
    path: &Path,
    stats: &mut OracleTraceSyncStats,
) -> Result<()> {
    let data = tokio::fs::read_to_string(&path).await?;
    let record: OracleTraceRecord = match serde_json::from_str(&data) {
        Ok(r) => r,
        Err(e) => {
            stats.failed += 1;
            stats.retained += 1;
            tracing::warn!(path = %path.display(), error = %e, "Invalid spool file");
            return Ok(());
        }
    };
    super::sync_upload::upload(remote, &record, path, stats).await;
    Ok(())
}
