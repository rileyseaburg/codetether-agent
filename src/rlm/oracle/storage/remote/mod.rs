//! Pluggable remote-upload backend for oracle traces.
//!
//! Defines [`OracleRemote`], the trait that storage backends
//! implement, and provides a concrete MinIO/S3 implementation
//! used for durable off-host persistence.
//!
//! # Examples
//!
//! ```ignore
//! let remote = MinioOracleRemote::from_bus_config(cfg)?;
//! let (key, url) = remote.upload_record(&record).await?;
//! ```

mod minio_backend;
mod minio_upload;

pub(super) use minio_backend::MinioOracleRemote;

use super::super::record::OracleTraceRecord;
use anyhow::Result;
use async_trait::async_trait;

/// Backend-agnostic interface for uploading oracle trace
/// records to durable remote storage.
///
/// Implementations should be idempotent — re-uploading an
/// identical record must overwrite rather than duplicate.
///
/// # Examples
///
/// ```ignore
/// let (key, url) = remote.upload_record(&record).await?;
/// println!("stored at {url}");
/// ```
#[async_trait]
pub(crate) trait OracleRemote: Send + Sync {
    /// Serialize and upload `record`, returning the object key
    /// and a human-readable URL to the stored artifact.
    async fn upload_record(&self, record: &OracleTraceRecord) -> Result<(String, String)>;
}
