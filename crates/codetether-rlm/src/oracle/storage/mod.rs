//! Oracle trace persistence with local spool + MinIO/S3 upload.
//!
//! Traces are first written atomically to a local spool directory,
//! then uploaded to MinIO/S3 when a remote backend is configured.
//! Failed uploads are retried on the next `sync_pending` pass.
//!
//! # Examples
//!
//! ```ignore
//! let storage = OracleTraceStorage::from_env_or_vault().await;
//! let result = storage.persist_result(&oracle_result).await?;
//! let stats = storage.sync_pending().await?;
//! ```

mod helpers;
mod manager;
mod remote;
mod spool_io;
mod sync;
mod sync_stats;
mod types;
mod usage;

pub use helpers::default_spool_dir;
pub use manager::OracleTraceStorage;
pub use sync_stats::OracleTraceSyncStats;
pub use types::OracleTracePersistResult;

#[cfg(test)]
mod tests;
