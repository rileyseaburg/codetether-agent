//! Rolling 1000-entry buffer of provider requests, plus per-provider aggregation.

use std::collections::HashMap;
use tokio::sync::Mutex;

use super::request::ProviderRequestRecord;
use super::snapshot::ProviderSnapshot;
use super::stats::{mean, percentile, sort_f64};

/// Bounded history of provider requests. Oldest entries are evicted once
/// the buffer exceeds 1000 records.
///
/// Prefer the [`crate::telemetry::PROVIDER_METRICS`] singleton. Constructing
/// your own is only useful in tests.
#[derive(Debug, Default)]
pub struct ProviderMetrics {
    requests: Mutex<Vec<ProviderRequestRecord>>,
}

impl ProviderMetrics {
    /// Build an empty buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a request, evicting the oldest if the buffer exceeds 1000.
    pub async fn record(&self, record: ProviderRequestRecord) {
        let mut requests = self.requests.lock().await;
        requests.push(record);
        if requests.len() > 1000 {
            requests.remove(0);
        }
    }

    /// Return up to `limit` most-recent records (newest first).
    pub async fn get_recent(&self, limit: usize) -> Vec<ProviderRequestRecord> {
        let requests = self.requests.lock().await;
        requests.iter().rev().take(limit).cloned().collect()
    }

    /// Aggregate the buffer into one [`ProviderSnapshot`] per provider.
    /// Returns an empty `Vec` if the buffer lock is contended.
    pub fn all_snapshots(&self) -> Vec<ProviderSnapshot> {
        let Ok(guard) = self.requests.try_lock() else {
            return Vec::new();
        };
        let requests = guard.clone();
        drop(guard);
        aggregate_by_provider(requests)
    }
}

/// Group `requests` by provider and fold each group into a [`ProviderSnapshot`].
fn aggregate_by_provider(requests: Vec<ProviderRequestRecord>) -> Vec<ProviderSnapshot> {
    let mut by_provider: HashMap<String, Vec<ProviderRequestRecord>> = HashMap::new();
    for req in requests {
        by_provider
            .entry(req.provider.clone())
            .or_default()
            .push(req);
    }
    by_provider
        .into_iter()
        .filter(|(_, reqs)| !reqs.is_empty())
        .map(|(provider, reqs)| snapshot_for(provider, &reqs))
        .collect()
}

/// Build one [`ProviderSnapshot`] from an already-grouped `reqs` slice.
fn snapshot_for(provider: String, reqs: &[ProviderRequestRecord]) -> ProviderSnapshot {
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
