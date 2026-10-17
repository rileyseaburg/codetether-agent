//! Provider history aggregation.

use super::*;

/// Build one [`ProviderSnapshot`] from an already-grouped `reqs` slice.
pub(super) fn snapshot_for(provider: String, reqs: &[ProviderRequestRecord]) -> ProviderSnapshot {
    let request_count = reqs.len();
    let total_input_tokens: u64 = reqs.iter().map(|r| r.input_tokens).sum();
    let total_output_tokens: u64 = reqs.iter().map(|r| r.output_tokens).sum();
    let avg_latency_ms = mean(&reqs.iter().map(|r| r.latency_ms as f64).collect::<Vec<_>>());

    let mut tps: Vec<f64> = reqs.iter().map(|r| r.tokens_per_second()).collect();
    sort_f64(&mut tps);
    let mut lat: Vec<f64> = reqs.iter().map(|r| r.latency_ms as f64).collect();
    sort_f64(&mut lat);

    ProviderSnapshot {
        provider,
        request_count,
        total_input_tokens,
        total_output_tokens,
        avg_tps: mean(&tps),
        avg_latency_ms,
        p50_tps: percentile(&tps, 0.50),
        p50_latency_ms: percentile(&lat, 0.50),
        p95_tps: percentile(&tps, 0.95),
        p95_latency_ms: percentile(&lat, 0.95),
    }
}
