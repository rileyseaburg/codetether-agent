//! Core storage manager definition and constructors.
//!
//! [`OracleTraceStorage`] owns the local spool directory and an
//! optional remote backend. Construction is via
//! [`from_env_or_vault`](OracleTraceStorage::from_env_or_vault).
//!
//! # Examples
//!
//! ```ignore
//! let storage = OracleTraceStorage::from_env_or_vault().await;
//! ```

mod internals;
mod persist;

use super::helpers::default_spool_dir;
use super::remote::{MinioOracleRemote, OracleRemote};
use crate::bus::s3_sink::BusS3SinkConfig;
use std::path::PathBuf;
use std::sync::Arc;

pub(in super::super) const DEFAULT_MAX_SPOOL_BYTES: u64 = 500 * 1024 * 1024;

/// Coordinates local-spool-first persistence with optional
/// upload to a MinIO/S3 remote backend.
///
/// Every trace is atomically written to the spool directory.
/// If a remote is configured, the record is uploaded and the
/// local copy removed on success.
///
/// # Examples
///
/// ```ignore
/// let storage = OracleTraceStorage::from_env_or_vault().await;
/// let result = storage.persist_result(&oracle_result).await?;
/// ```
pub struct OracleTraceStorage {
    pub(in super::super) spool_dir: PathBuf,
    pub(in super::super) max_spool_bytes: u64,
    pub(in super::super) remote: Option<Arc<dyn OracleRemote>>,
}

impl OracleTraceStorage {
    /// Build from environment variables and Vault-backed MinIO
    /// credentials, falling back to local-only mode on failure.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let storage = OracleTraceStorage::from_env_or_vault().await;
    /// ```
    pub async fn from_env_or_vault() -> Self {
        let spool_dir = default_spool_dir();
        let max = std::env::var("CODETETHER_ORACLE_SPOOL_MAX_BYTES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_MAX_SPOOL_BYTES);
        let remote = match BusS3SinkConfig::from_env_or_vault().await {
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

    /// Construct with injected dependencies for testing.
    #[cfg(test)]
    pub(in super::super) fn new_for_test(
        spool_dir: PathBuf,
        max_spool_bytes: u64,
        remote: Option<Arc<dyn OracleRemote>>,
    ) -> Self {
        Self {
            spool_dir,
            max_spool_bytes,
            remote,
        }
    }
}
