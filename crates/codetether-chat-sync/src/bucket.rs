//! Ensure the configured archive bucket exists.
use anyhow::Result;
use minio::s3::{client::MinioClient, types::S3Api};

/// Create the destination bucket when absent.
/// # Arguments
/// S3 client and destination bucket name.
/// # Returns
/// Unit after the bucket exists.
/// # Errors
/// Propagates bucket lookup or creation errors.
/// # Examples
/// ```text
/// Configure the archive destination, then use ensure_minio_bucket through the chat-sync API.
/// ```
pub async fn ensure_minio_bucket(client: &MinioClient, bucket: &str) -> Result<()> {
    let resp = client.bucket_exists(bucket)?.build().send().await?;
    if !resp.exists() {
        client.create_bucket(bucket)?.build().send().await?;
        tracing::info!(bucket = %bucket, "Created MinIO bucket");
    }
    Ok(())
}
