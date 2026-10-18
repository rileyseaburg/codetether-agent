//! Background transport-health probe task for the worker.
//!
//! reqwest hides the stream `RawFd`, so we periodically probe the server's
//! `host:port` on a dedicated socket, sample `TCP_INFO`, and record it to
//! `TRANSPORT_METRICS`. See `docs/transport-first-class-plan.md` Phase 4.

use super::{PROBE_INTERVAL, probe_once, probe_target};
use codetether_telemetry::TRANSPORT_METRICS;

/// Spawn a task that periodically probes the server path and records the
/// latest `TCP_INFO` snapshot. Call `abort()` on the handle to stop the task.
///
/// # Arguments
/// * `server_url` — HTTP or HTTPS server base URL used to choose the probe target.
/// # Returns
/// A background task handle; an unsupported URL makes the task exit immediately.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let probe = codetether_a2a_stream::probe::spawn_transport_probe("http://localhost:8001");
/// probe.abort();
/// # }
/// ```
pub fn spawn_transport_probe(server_url: &str) -> tokio::task::JoinHandle<()> {
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
