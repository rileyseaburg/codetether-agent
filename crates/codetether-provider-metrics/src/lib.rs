//! Provider request and stream instrumentation over shared telemetry.
//!
//! [MetricsProvider] wraps a provider without changing its completion interface.
//! Timing, token accounting, and request identity use shared core services.
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, EmbeddingRequest, EmbeddingResponse, ModelInfo,
    Provider, StreamChunk, Usage,
};
mod metrics;
pub use metrics::MetricsProvider;

mod session_cost;
pub use session_cost::session_cost_usd;
