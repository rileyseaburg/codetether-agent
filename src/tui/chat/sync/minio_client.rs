//! MinIO/S3 client construction.

use anyhow::Result;
use minio::s3::Client as MinioClient;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::S3Api;

use super::config_types::ChatSyncConfig;

pub fn build_minio_client(endpoint: &str, config: &ChatSyncConfig) -> Result<MinioClient> {
    let base_url: BaseUrl = endpoint.parse()?;
    let provider = StaticProvider::new(&config.access_key, &config.secret_key, None);
    let client = MinioClient::new(
        base_url,
        Some(Box::new(provider)),
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

pub async fn ensure_minio_bucket(client: &MinioClient, bucket: &str) -> Result<()> {
    let resp = client.bucket_exists(bucket).send().await?;
    if !resp.exists {
        client.create_bucket(bucket).send().await?;
        tracing::info!(bucket = %bucket, "Created MinIO bucket");
    }
    Ok(())
}
