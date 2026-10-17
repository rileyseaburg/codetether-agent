//! from env for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    #[allow(dead_code)]
    /// Create sink from environment variables
    pub async fn from_env(bus: Arc<AgentBus>) -> Result<Self> {
        let config = BusS3SinkConfig::from_env()?;
        Self::from_config(bus, config).await
    }
}
