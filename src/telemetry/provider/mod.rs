//! # Provider telemetry
//!
//! Tracks every LLM provider request the agent makes, in a rolling 1000-entry
//! buffer, and derives per-provider averages / percentiles from it.
//!
//! | File | Responsibility |
//! |---|---|
//! | [`request`]  | [`ProviderRequestRecord`] — one request's raw timing/tokens |
//! | [`snapshot`] | [`ProviderSnapshot`] — aggregated view per provider |
//! | [`metrics`]  | [`ProviderMetrics`] — rolling buffer + aggregation entry point |
//! | [`stats`]    | internal percentile / mean helpers used by `metrics` |

pub mod metrics;
pub mod request;
pub mod snapshot;
pub mod stats;

pub use metrics::ProviderMetrics;
pub use request::ProviderRequestRecord;
pub use snapshot::ProviderSnapshot;
