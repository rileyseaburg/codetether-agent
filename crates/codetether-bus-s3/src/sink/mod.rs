//! S3 client lifecycle, batching and archival upload.
//!
//! Construct [BusS3Sink] from resolved configuration and call [BusS3Sink::run].
use super::*;

// ─── S3 Sink ─────────────────────────────────────────────────────────────

/// S3 sink that archives bus messages as JSONL training records.
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_bus_core::AgentBus;
/// use codetether_bus_s3::{BusS3Sink, BusS3SinkConfig};
/// let config = BusS3SinkConfig::from_env()?;
/// let sink = BusS3Sink::from_config(AgentBus::new().into_arc(), config).await?;
/// sink.run().await?;
/// # Ok(())
/// # }
/// ```
pub struct BusS3Sink {
    _bus: Arc<AgentBus>,
    client: MinioClient,
    config: BusS3SinkConfig,
    rx: broadcast::Receiver<BusEnvelope>,
}
mod access;
mod bucket;
mod client;
mod construct;
mod endpoint;
mod flush;
mod from_env;
mod interval;
mod receive;
mod run;
mod upload;
use endpoint::normalize_endpoint;
