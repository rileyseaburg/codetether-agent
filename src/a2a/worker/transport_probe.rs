//! Background transport-health probe task for the worker.
//!
//! reqwest hides the stream `RawFd`, so we periodically probe the server's
//! `host:port` on a dedicated socket, sample `TCP_INFO`, and record it to
//! `TRANSPORT_METRICS`. See `docs/transport-first-class-plan.md` Phase 4.

use crate::a2a::stream::probe::{PROBE_INTERVAL, probe_once, probe_target};
use crate::telemetry::TRANSPORT_METRICS;

/// Spawn a task that periodically probes the server path and records the
/// latest `TCP_INFO` snapshot. The returned handle aborts the probe on drop.
pub(super) fn spawn_transport_probe(server_url: &str) -> tokio::task::JoinHandle<()> {
    let target = probe_target(server_url);
    tokio::spawn(async move {
        let Some(target) = target else { return };
        let mut interval = tokio::time::interval(PROBE_INTERVAL);
        loop {
            interval.tick().await;
            if let Some(snapshot) = probe_once(&target).await {
                TRANSPORT_METRICS.record(snapshot);
            }
        }
    })
}
