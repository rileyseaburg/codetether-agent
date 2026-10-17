//! Compatibility exports for shared provider instrumentation and request identity.
pub use codetether_provider_metrics::MetricsProvider;
#[path = "metrics/identity.rs"]
pub(crate) mod identity;
