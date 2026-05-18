//! Context types matching the original API so call sites are unchanged.
//!
//! `AutoProcessContext` keeps the same concrete fields (`Arc<dyn Provider>`,
//! `Option<SessionBus>`, etc.) so every existing construction site compiles
//! without modification.

use std::sync::Arc;
use uuid::Uuid;

use crate::provider::Provider;
use crate::session::SessionBus;

/// Progress tick during RLM auto-processing.
pub use codetether_rlm::router::ProcessProgress;

/// Context for a routing decision.
pub use codetether_rlm::router::RoutingContext;

/// Outcome of a routing decision.
pub use codetether_rlm::router::RoutingResult;

/// Context for auto-processing — concrete host types.
///
/// Thinly mirrors the crate's internal `CrateAutoProcessContext` but
/// holds the concrete `Provider` and `SessionBus` so downstream code
/// (session helpers, compression) doesn't need trait-object wrapping.
pub struct AutoProcessContext<'a> {
    pub tool_id: &'a str,
    pub tool_args: serde_json::Value,
    pub session_id: &'a str,
    pub abort: Option<tokio::sync::watch::Receiver<bool>>,
    pub on_progress: Option<Box<dyn Fn(ProcessProgress) + Send + Sync>>,
    pub provider: Arc<dyn Provider>,
    pub model: String,
    pub bus: Option<SessionBus>,
    pub trace_id: Option<Uuid>,
    pub subcall_provider: Option<Arc<dyn Provider>>,
    pub subcall_model: Option<String>,
}
