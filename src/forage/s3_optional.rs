//! Best-effort S3 bus archival startup for forage.
//!
//! Plain `codetether forage` must run with zero external dependencies, so a
//! missing S3/MinIO configuration degrades to local-only mode with a warning
//! rather than aborting the run.

use crate::bus::AgentBus;
use crate::bus::s3_sink::{BusS3Sink, BusS3SinkConfig};
use anyhow::Result;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// Attempt to start the S3 bus sink.
///
/// Returns `Some(handle)` when S3 is configured and the sink started, or `None`
/// when no configuration is present (caller continues in local-only mode).
pub(super) async fn try_start_bus_s3_sink(bus: Arc<AgentBus>) -> Option<JoinHandle<Result<()>>> {
    let config = match BusS3SinkConfig::from_env_or_vault().await {
        Ok(config) => config,
        Err(err) => {
            tracing::warn!(
                error = %err,
                "S3 bus archival not configured; running forage in local-only mode. \
                 Set MINIO_*/CODETETHER_CHAT_SYNC_MINIO_* or Vault 'chat-sync-minio' to enable."
            );
            return None;
        }
    };
    match BusS3Sink::from_config(bus, config).await {
        Ok(sink) => Some(tokio::spawn(async move { sink.run().await })),
        Err(err) => {
            tracing::warn!(error = %err, "Failed to start S3 bus sink; continuing local-only");
            None
        }
    }
}
