//! Environment aliases, defaults, and legacy configuration warning.

use super::S3SinkConfig;
fn setting(primary: &str, legacy: &str) -> Result<String, std::env::VarError> {
    std::env::var(primary).or_else(|_| std::env::var(legacy))
}
impl S3SinkConfig {
    /// Create config from environment variables
    ///
    /// Supports both `S3_*` (preferred) and `CODETETHER_S3_*` (legacy) environment variables.
    /// The `S3_*` variables take precedence over `CODETETHER_S3_*` variables for backwards compatibility.
    ///
    /// # Environment Variables
    /// - `S3_BUCKET` (or `CODETETHER_S3_BUCKET`): S3 bucket name
    /// - `S3_PREFIX` (or `CODETETHER_S3_PREFIX`): Path prefix for uploads (default: "events/")
    /// - `S3_ENDPOINT` (or `CODETETHER_S3_ENDPOINT`): Custom endpoint for R2/MinIO
    /// - `S3_ACCESS_KEY` (or `CODETETHER_S3_ACCESS_KEY`): Access key
    /// - `S3_SECRET_KEY` (or `CODETETHER_S3_SECRET_KEY`): Secret key
    /// - `S3_REGION` (or `CODETETHER_S3_REGION`, `AWS_REGION`): Region (default: "us-east-1")
    /// - `S3_STUB_MODE` (or `CODETETHER_S3_STUB_MODE`): Enable stub mode (skip actual uploads)
    pub fn from_env() -> Option<Self> {
        // Try S3_* first (new naming), fall back to CODETETHER_S3_* (legacy)
        let bucket = setting("S3_BUCKET", "CODETETHER_S3_BUCKET").ok()?;

        let prefix =
            setting("S3_PREFIX", "CODETETHER_S3_PREFIX").unwrap_or_else(|_| "events/".to_string());

        let endpoint = setting("S3_ENDPOINT", "CODETETHER_S3_ENDPOINT").ok();

        let access_key = setting("S3_ACCESS_KEY", "CODETETHER_S3_ACCESS_KEY").ok();

        let secret_key = setting("S3_SECRET_KEY", "CODETETHER_S3_SECRET_KEY").ok();

        let region = setting("S3_REGION", "CODETETHER_S3_REGION")
            .or_else(|_| std::env::var("AWS_REGION"))
            .unwrap_or_else(|_| "us-east-1".to_string());

        let stub_mode = setting("S3_STUB_MODE", "CODETETHER_S3_STUB_MODE")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        // Log deprecation warning when using legacy CODETETHER_S3_* variables
        if std::env::var("CODETETHER_S3_BUCKET").is_ok() && std::env::var("S3_BUCKET").is_err() {
            tracing::warn!(
                "Using legacy CODETETHER_S3_* environment variables. These are deprecated. Please migrate to S3_* naming convention."
            );
        }

        Some(Self {
            bucket,
            prefix,
            endpoint,
            access_key,
            secret_key,
            region,
            stub_mode,
        })
    }
}
