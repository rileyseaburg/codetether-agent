//! MinIO/S3 backend struct and constructor.
//!
//! The actual [`OracleRemote`](super::OracleRemote) upload logic
//! lives in the sibling [`super::minio_upload`] module.
//!
//! # Examples
//!
//! ```ignore
//! let backend = MinioOracleRemote::from_bus_config(cfg)?;
//! ```

use crate::S3Config;
use anyhow::{Context, Result};
use minio::s3::client::{MinioClient, MinioClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use std::str::FromStr;

use super::super::helpers::normalize_endpoint;

/// Uploads oracle traces to a MinIO-compatible S3 bucket.
///
/// Objects are partitioned by verdict and date under the
/// configured prefix so that listing golden traces for a
/// given day is a single `ListObjectsV2` call.
///
/// # Examples
///
/// ```ignore
/// let backend = MinioOracleRemote::from_bus_config(cfg)?;
/// let (key, url) = backend.upload_record(&record).await?;
/// ```
pub(in super::super) struct MinioOracleRemote {
    pub(super) client: MinioClient,
    pub(super) endpoint: String,
    pub(super) bucket: String,
    pub(super) prefix: String,
    pub(super) region: String,
}

impl MinioOracleRemote {
    /// Build from the shared bus S3 sink configuration.
    ///
    /// Reads region from `CODETETHER_BUS_S3_REGION`,
    /// `MINIO_REGION`, or `AWS_REGION` (in that order),
    /// defaulting to `us-east-1`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let backend = MinioOracleRemote::from_bus_config(cfg)?;
    /// ```
    pub fn from_bus_config(config: S3Config) -> Result<Self> {
        let endpoint = normalize_endpoint(&config.endpoint, config.secure);
        let creds = StaticProvider::new(&config.access_key, &config.secret_key, None);
        let base_url = BaseUrl::from_str(&endpoint)
            .with_context(|| format!("Invalid MinIO endpoint: {endpoint}"))?;
        let client = MinioClientBuilder::new(base_url)
            .provider(Some(creds))
            .build()
            .context("Failed to build MinIO client")?;
        let region = std::env::var("CODETETHER_BUS_S3_REGION")
            .or_else(|_| std::env::var("MINIO_REGION"))
            .or_else(|_| std::env::var("AWS_REGION"))
            .unwrap_or_else(|_| "us-east-1".to_string());
        Ok(Self {
            client,
            endpoint,
            bucket: config.bucket,
            prefix: config.prefix,
            region,
        })
    }
}
