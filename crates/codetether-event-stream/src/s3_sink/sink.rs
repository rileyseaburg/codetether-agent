//! Archival client state and read-only configuration access.

use super::S3SinkConfig;
use reqwest::Client;

/// Archives event payloads through an HTTP client and S3-compatible settings.
///
/// # Examples
/// ```no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_event_stream::s3_sink::{S3Sink, S3SinkConfig};
/// let config = serde_json::from_str::<S3SinkConfig>(r#"{"bucket":"logs","prefix":"events/","stub_mode":true}"#).unwrap();
/// let sink = S3Sink::from_config(config).await.unwrap();
/// assert_eq!(sink.bucket_name(), "logs");
/// # });
/// ```
pub struct S3Sink {
    pub(super) client: Client,
    pub(super) config: S3SinkConfig,
}
impl S3Sink {
    /// Check if S3 sink is configured
    ///
    /// Returns true if either `CODETETHER_S3_BUCKET` or `S3_BUCKET` is set.
    pub fn is_configured() -> bool {
        std::env::var("CODETETHER_S3_BUCKET")
            .or_else(|_| std::env::var("S3_BUCKET"))
            .is_ok()
    }

    #[allow(dead_code)]
    /// Get the bucket name
    pub fn bucket_name(&self) -> &str {
        &self.config.bucket
    }

    #[allow(dead_code)]
    /// Get the prefix
    pub fn prefix(&self) -> &str {
        &self.config.prefix
    }
}
