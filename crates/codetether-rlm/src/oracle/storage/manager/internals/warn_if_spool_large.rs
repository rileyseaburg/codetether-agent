//! Focused storage helper.
use crate::oracle::storage::{manager::OracleTraceStorage, usage::spool_usage_bytes};

impl OracleTraceStorage {
    pub(in crate::oracle::storage) async fn warn_if_spool_large(&self) {
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
