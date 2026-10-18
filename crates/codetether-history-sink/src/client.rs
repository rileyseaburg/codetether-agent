//! Build the MinIO client using owned history credentials and endpoint.
use crate::HistorySinkConfig;
use anyhow::{Context, Result};
use minio::s3::{
    client::{MinioClient, MinioClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
};
use std::str::FromStr;

/// Build a MinIO client from a [`HistorySinkConfig`].
pub(crate) fn build_client(config: &HistorySinkConfig) -> Result<MinioClient> {
    let base_url = BaseUrl::from_str(&config.endpoint)
        .with_context(|| format!("Invalid MinIO endpoint: {}", config.endpoint))?;
    let creds = StaticProvider::new(&config.access_key, &config.secret_key, None);
    MinioClientBuilder::new(base_url)
        .provider(Some(creds))
        .build()
        .context("Failed to build MinIO client for history sink")
}
