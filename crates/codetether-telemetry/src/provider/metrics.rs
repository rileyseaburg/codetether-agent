//! Rolling 1000-entry buffer of provider requests, plus per-provider aggregation.

use std::collections::HashMap;
use tokio::sync::Mutex;

use super::request::ProviderRequestRecord;
use super::snapshot::ProviderSnapshot;
use super::stats::{mean, percentile, sort_f64};

/// Bounded history of provider requests. Oldest entries are evicted once
/// the buffer exceeds 1000 records.
///
/// Prefer the [`crate::PROVIDER_METRICS`] singleton. Constructing
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
mod aggregate_by_provider;
mod snapshot_for;
use aggregate_by_provider::aggregate_by_provider;
use snapshot_for::snapshot_for;
