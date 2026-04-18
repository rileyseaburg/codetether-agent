//! MinIO/S3 client construction.

use anyhow::Result;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::Client as MinioClient;

use super::config::ChatSyncConfig;

pub fn build_minio_client(endpoint: &str, config: &ChatSyncConfig) -> Result<MinioClient> {
    let base_url = BaseUrl::from(endpoint)?;
    let provider = StaticProvider::new(&config.access_key, &config.secret_key, None);
    let mut builder = MinioClient::builder()
        .with_endpoint(base_url.clone())
        .with_provider(Some(provider))
        .bucket_name(&config.bucket);

    if config.ignore_cert_check {
        builder = builder.secure(false);
    }

    tracing::debug!(
        endpoint = %endpoint,
        bucket = %config.bucket,
        ignore_cert = config.ignore_cert_check,
        "Built MinIO client"
    );

    Ok(builder.build()?)
}

pub async fn ensure_minio_bucket(client: &MinioClient, bucket: &str) -> Result<()> {
    let exists = client.bucket_exists(bucket).send().await?;
    if !exists {
        client.make_bucket(bucket).send().await?;
        tracing::info!(bucket = %bucket, "Created MinIO bucket");
    }
    Ok(())
}
