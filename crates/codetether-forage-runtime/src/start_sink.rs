//! Start optional S3 archival without changing run ownership.
use super::*;
use std::sync::Arc;
pub(super) async fn start(
    args: &ForageArgs,
    bus: &Arc<AgentBus>,
) -> Option<tokio::task::JoinHandle<Result<()>>> {
    if args.no_s3 {
        tracing::info!(target: "codetether_agent::forage", "S3 archival disabled (--no-s3); running forage in local-only mode");
        None
    } else {
        s3_optional::try_start_bus_s3_sink(bus.clone()).await
    }
}
