//! MinIO/S3 client construction.

use anyhow::Result;
use minio::s3::client::MinioClient;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;

use super::config_types::ChatSyncConfig;

/// Construct the configured S3-compatible client.
/// # Arguments
/// Endpoint and credential/TLS configuration.
/// # Returns
/// A configured MinIO client.
/// # Errors
/// Returns endpoint or client construction errors.
/// # Examples
/// ```text
/// Configure the archive destination, then use build_minio_client through the chat-sync API.
/// ```
pub fn build_minio_client(endpoint: &str, config: &ChatSyncConfig) -> Result<MinioClient> {
    let base_url: BaseUrl = endpoint.parse()?;
    let provider = StaticProvider::new(&config.access_key, &config.secret_key, None);
    let client = MinioClient::new(
        base_url,
        Some(provider),
        None,
        if config.ignore_cert_check {
            Some(true)
        } else {
            None
        },
    )?;

    tracing::debug!(
        endpoint = %endpoint,
        bucket = %config.bucket,
        ignore_cert = config.ignore_cert_check,
        "Built MinIO client"
    );

    Ok(client)
}
pub use crate::bucket::ensure_minio_bucket;
