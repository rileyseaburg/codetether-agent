//! S3 connection, batching and credential-source configuration.
//!
//! Build a [BusS3SinkConfig] explicitly or resolve environment/Vault sources
//! with [BusS3SinkConfig::from_env_or_vault].
use super::*;

/// Configuration for the bus S3 sink.
///
/// # Examples
/// ```
/// use codetether_bus_s3::BusS3SinkConfig;
/// let config = BusS3SinkConfig {
///     endpoint: "http://localhost:9000".into(),
///     access_key: String::new(), secret_key: String::new(),
///     bucket: "training".into(), prefix: "bus/".into(),
///     batch_size: 100, flush_interval_secs: 30,
///     secure: false, ignore_cert: false,
/// };
/// assert_eq!(config.batch_size, 100);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusS3SinkConfig {
    /// MinIO/S3 endpoint URL (e.g., "http://localhost:9000")
    pub endpoint: String,
    /// Access key
    pub access_key: String,
    /// Secret key
    pub secret_key: String,
    /// Bucket name for bus logs
    pub bucket: String,
    /// Path prefix within the bucket
    #[serde(default = "default_prefix")]
    pub prefix: String,
    /// Batch size before flushing to S3
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    /// Max batch age in seconds before flushing
    #[serde(default = "default_flush_interval_secs")]
    pub flush_interval_secs: u64,
    /// Whether to use SSL/TLS
    #[serde(default)]
    pub secure: bool,
    /// Whether to ignore certificate errors (for self-signed certs)
    #[serde(default)]
    pub ignore_cert: bool,
}
mod defaults;
use defaults::*;
mod chat_sync;
mod from_env;
mod from_sources;
mod helpers;
mod vault;
use helpers::*;
#[cfg(test)]
mod tests;
