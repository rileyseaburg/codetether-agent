//! Focused storage helper.
use crate::oracle::{record::OracleTraceRecord, storage::manager::OracleTraceStorage};

impl OracleTraceStorage {
    pub(in crate::oracle::storage) async fn try_upload(
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
}
