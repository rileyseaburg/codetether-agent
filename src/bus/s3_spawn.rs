//! Background task launcher for the bus S3 training sink.

use super::{AgentBus, BusS3Sink, BusS3SinkConfig};
use std::sync::Arc;
use tracing::{error, warn};

/// Spawn the bus S3 sink in a non-blocking background task.
pub fn spawn_bus_s3_sink(bus: Arc<AgentBus>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        match BusS3SinkConfig::from_env_or_vault().await {
            Ok(config) => run_configured(bus, config).await,
            Err(e) => warn!(
                error = %e,
                "Bus S3 sink not configured - set MinIO env vars or Vault"
            ),
        }
    })
}

async fn run_configured(bus: Arc<AgentBus>, config: BusS3SinkConfig) {
    match BusS3Sink::from_config(bus, config).await {
        Ok(sink) => {
            if let Err(e) = sink.run().await {
                error!(error = %e, "Bus S3 sink failed");
            }
        }
        Err(e) => error!(error = %e, "Bus S3 sink failed to initialize"),
    }
}
