//! Provider metric record orchestration and storage.

use super::super::Usage;
use crate::telemetry::PROVIDER_METRICS;

#[path = "record/build.rs"]
mod build;
#[path = "record/log.rs"]
mod log;

/// Records a completed non-streaming provider request.
pub(super) async fn request(
    provider: &str,
    model: &str,
    start: std::time::Instant,
    usage: &Usage,
    success: bool,
) {
    let record = build::new(provider, model, start, None, Some(usage), success);
    log::write(&record, "Provider request completed");
    PROVIDER_METRICS.record(record).await;
}

/// Records a completed or failed provider stream.
pub(super) async fn stream(
    provider: &str,
    model: &str,
    start: std::time::Instant,
    ttft_ms: Option<u64>,
    usage: Option<&Usage>,
    success: bool,
) {
    let record = build::new(provider, model, start, ttft_ms, usage, success);
    log::write(&record, "Provider streaming request completed");
    PROVIDER_METRICS.record(record).await;
}
