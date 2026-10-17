//! construct for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    /// Create a new bus S3 sink
    pub async fn new(
        bus: Arc<AgentBus>,
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
        bucket: &str,
        prefix: &str,
    ) -> Result<Self> {
        let config = BusS3SinkConfig {
            endpoint: endpoint.to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            bucket: bucket.to_string(),
            prefix: prefix.to_string(),
            batch_size: 100,
            flush_interval_secs: 30,
            secure: endpoint.starts_with("https"),
            ignore_cert: false,
        };

        Self::from_config(bus, config).await
    }
}
