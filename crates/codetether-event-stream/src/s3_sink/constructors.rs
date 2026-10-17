//! Create archival clients from explicit or environment configuration.

use super::{S3Sink, S3SinkConfig, S3SinkError};
use reqwest::Client;
impl S3Sink {
    #[allow(dead_code)]
    /// Create a new S3 sink
    ///
    /// # Arguments
    /// Creates default-region settings from the supplied bucket, prefix, endpoint, and credentials.
    ///
    /// # Returns
    /// The configured archival client.
    ///
    /// # Errors
    /// Returns an error if the HTTP client cannot be created.
    ///
    /// # Examples
    /// ```no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_event_stream::s3_sink::{S3Sink, S3SinkConfig};
    /// let sink = S3Sink::new("logs".into(), "events/".into(), None, None, None).await.unwrap();
    /// assert_eq!(sink.bucket_name(), "logs");
    /// # });
    /// ```
    pub async fn new(
        bucket: String,
        prefix: String,
        endpoint: Option<String>,
        access_key: Option<String>,
        secret_key: Option<String>,
    ) -> Result<Self, S3SinkError> {
        let config = S3SinkConfig {
            bucket,
            prefix,
            endpoint,
            access_key,
            secret_key,
            region: "us-east-1".to_string(),
            stub_mode: false,
        };

        Self::from_config(config).await
    }

    /// Create S3 sink from configuration
    ///
    /// # Arguments
    /// Creates an HTTP client using the supplied archival settings.
    ///
    /// # Returns
    /// The configured archival client.
    ///
    /// # Errors
    /// Returns an error if the HTTP client cannot be created.
    ///
    /// # Examples
    /// ```no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_event_stream::s3_sink::{S3Sink, S3SinkConfig};
    /// let config = serde_json::from_str::<S3SinkConfig>(r#"{"bucket":"logs","prefix":"events/","stub_mode":true}"#).unwrap();
    /// let sink = S3Sink::from_config(config).await.unwrap();
    /// assert_eq!(sink.prefix(), "events/");
    /// # });
    /// ```
    pub async fn from_config(config: S3SinkConfig) -> Result<Self, S3SinkError> {
        let client = Client::builder().build().map_err(S3SinkError::Http)?;

        Ok(Self { client, config })
    }

    /// Create S3 sink from environment variables
    ///
    /// # Arguments
    /// Reads configured bucket settings and creates an HTTP client.
    ///
    /// # Returns
    /// The configured archival client.
    ///
    /// # Errors
    /// Returns an error for missing bucket configuration or HTTP-client creation failure.
    ///
    /// # Examples
    /// ```no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_event_stream::s3_sink::{S3Sink, S3SinkConfig};
    /// let result = S3Sink::from_env().await;
    /// assert!(result.is_ok() || std::env::var("S3_BUCKET").is_err());
    /// # });
    /// ```
    pub async fn from_env() -> Result<Self, S3SinkError> {
        let config = S3SinkConfig::from_env().ok_or_else(|| {
            S3SinkError::MissingConfig("S3_BUCKET (or CODETETHER_S3_BUCKET) not set".to_string())
        })?;

        Self::from_config(config).await
    }
}
