//! Spool location, capacity, and optional upload backend.

use super::super::remote::OracleRemote;
use std::{path::PathBuf, sync::Arc};

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
///
/// # Examples
/// ```no_run
/// use codetether_rlm::{OracleTraceStorage};
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let storage = OracleTraceStorage::from_env_or_vault().await;
/// assert!(storage.sync_pending().await.is_ok());
/// # });
/// ```
pub struct OracleTraceStorage {
    pub(in crate::oracle) spool_dir: PathBuf,
    pub(in crate::oracle) max_spool_bytes: u64,
    pub(in crate::oracle) remote: Option<Arc<dyn OracleRemote>>,
}
