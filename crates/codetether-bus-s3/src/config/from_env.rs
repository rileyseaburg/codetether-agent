//! from env for the S3 bus adapter.
use super::*;
impl BusS3SinkConfig {
    /// Create config from environment variables
    ///
    /// Required:
    /// - `MINIO_ENDPOINT` or `CODETETHER_BUS_S3_ENDPOINT`
    /// - `MINIO_ACCESS_KEY` or `CODETETHER_BUS_S3_ACCESS_KEY`
    /// - `MINIO_SECRET_KEY` or `CODETETHER_BUS_S3_SECRET_KEY`
    ///
    /// Optional:
    /// - `CODETETHER_BUS_S3_BUCKET` (default: "codetether-training")
    /// - `CODETETHER_BUS_S3_PREFIX` (default: "bus/")
    pub fn from_env() -> Result<Self> {
        let endpoint = std::env::var("MINIO_ENDPOINT")
            .or_else(|_| std::env::var("CODETETHER_BUS_S3_ENDPOINT"))
            .context("MINIO_ENDPOINT or CODETETHER_BUS_S3_ENDPOINT required for bus S3 sink")?;

        let access_key = std::env::var("MINIO_ACCESS_KEY")
            .or_else(|_| std::env::var("CODETETHER_BUS_S3_ACCESS_KEY"))
            .context("MINIO_ACCESS_KEY or CODETETHER_BUS_S3_ACCESS_KEY required")?;

        let secret_key = std::env::var("MINIO_SECRET_KEY")
            .or_else(|_| std::env::var("CODETETHER_BUS_S3_SECRET_KEY"))
            .context("MINIO_SECRET_KEY or CODETETHER_BUS_S3_SECRET_KEY required")?;

        Ok(Self {
            endpoint,
            access_key,
            secret_key,
            bucket: std::env::var("CODETETHER_BUS_S3_BUCKET")
                .unwrap_or_else(|_| "codetether-training".to_string()),
            prefix: std::env::var("CODETETHER_BUS_S3_PREFIX")
                .unwrap_or_else(|_| "training/".to_string()),
            batch_size: std::env::var("CODETETHER_BUS_S3_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            flush_interval_secs: std::env::var("CODETETHER_BUS_S3_FLUSH_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            secure: std::env::var("MINIO_SECURE")
                .ok()
                .map(|s| s.to_lowercase() == "true")
                .unwrap_or(false),
            ignore_cert: std::env::var("MINIO_IGNORE_CERT")
                .ok()
                .map(|s| s.to_lowercase() == "true")
                .unwrap_or(false),
        })
    }
}
