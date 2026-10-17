//! Account for one remote upload and local cleanup.

use super::{remote::OracleRemote, sync_stats::OracleTraceSyncStats};
use crate::oracle::record::OracleTraceRecord;
use std::path::Path;
pub(super) async fn upload(
    remote: &dyn OracleRemote,
    record: &OracleTraceRecord,
    path: &Path,
    stats: &mut OracleTraceSyncStats,
) {
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
