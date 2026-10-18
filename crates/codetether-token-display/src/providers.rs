//! Detailed provider throughput and latency percentiles.
use codetether_telemetry::PROVIDER_METRICS;
pub(crate) fn append(lines: &mut Vec<String>) {
    // Provider performance metrics (TPS, latency)
    let provider_snapshots = PROVIDER_METRICS.all_snapshots();
    if !provider_snapshots.is_empty() {
        lines.push("  PROVIDER PERFORMANCE:".to_string());

        for snapshot in provider_snapshots.iter().take(5) {
            if snapshot.request_count > 0 {
                lines.push(format!(
                    "    {}: {:.1} avg TPS | {:.0}ms avg latency | {} reqs",
                    snapshot.provider,
                    snapshot.avg_tps,
                    snapshot.avg_latency_ms,
                    snapshot.request_count
                ));

                // Show p50/p95 if we have enough requests
                if snapshot.request_count >= 5 {
                    lines.push(format!(
                        "      p50: {:.1} TPS / {:.0}ms | p95: {:.1} TPS / {:.0}ms",
                        snapshot.p50_tps,
                        snapshot.p50_latency_ms,
                        snapshot.p95_tps,
                        snapshot.p95_latency_ms
                    ));
                }
            }
        }
        lines.push("".to_string());
    }
}
