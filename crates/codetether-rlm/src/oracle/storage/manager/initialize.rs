//! Resolve spool configuration and optional remote backend.

use super::super::{
    helpers::default_spool_dir,
    remote::{MinioOracleRemote, OracleRemote},
};
use super::{DEFAULT_MAX_SPOOL_BYTES, OracleTraceStorage};
use crate::S3Config;
use std::sync::Arc;
impl OracleTraceStorage {
    /// Build from environment variables and Vault-backed MinIO
    /// credentials, falling back to local-only mode on failure.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let storage = OracleTraceStorage::from_env_or_vault().await;
    /// ```
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_rlm::{OracleTraceStorage};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let storage = OracleTraceStorage::from_env_or_vault().await;
    /// assert!(storage.sync_pending().await.is_ok());
    /// # });
    /// ```
    pub async fn from_env_or_vault() -> Self {
        let spool_dir = default_spool_dir();
        let max = std::env::var("CODETETHER_ORACLE_SPOOL_MAX_BYTES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_MAX_SPOOL_BYTES);
        let remote = match S3Config::from_env_or_vault() {
            Ok(cfg) => match MinioOracleRemote::from_bus_config(cfg) {
                Ok(r) => Some(Arc::new(r) as Arc<dyn OracleRemote>),
                Err(e) => {
                    tracing::warn!(error = %e, "Remote init failed");
                    None
                }
            },
            Err(e) => {
                tracing::warn!(error = %e, "Remote not configured");
                None
            }
        };
        Self {
            spool_dir,
            max_spool_bytes: max,
            remote,
        }
    }
}
