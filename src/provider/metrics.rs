//! Provider wrapper that records completion and stream performance metrics.

use super::Provider;
use std::sync::Arc;

#[path = "metrics/calls.rs"]
mod calls;
#[path = "metrics/provider_impl.rs"]
mod provider_impl;
#[path = "metrics/provider_impl_delegates.rs"]
mod provider_impl_delegates;
#[path = "metrics/record.rs"]
mod record;
#[path = "metrics/stream.rs"]
mod stream;

/// A provider wrapper that instruments every request.
pub struct MetricsProvider {
    pub(super) inner: Arc<dyn Provider>,
}

impl MetricsProvider {
    /// Wraps a provider with automatic latency and throughput collection.
    pub fn wrap(inner: Arc<dyn Provider>) -> Arc<Self> {
        Arc::new(Self { inner })
    }
}
