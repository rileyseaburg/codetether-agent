//! Application routing context with an explicit event-bus type.
//!
//! Provider handles and callbacks retain their original ownership; only the
//! event transport is supplied by the embedding application.

use std::sync::Arc;
use uuid::Uuid;

use codetether_provider_core::Provider;

/// Progress tick during RLM auto-processing.
pub use codetether_rlm::router::ProcessProgress;

/// Context for a routing decision.
pub use codetether_rlm::router::RoutingContext;

/// Outcome of a routing decision.
pub use codetether_rlm::router::RoutingResult;

/// Inputs retained while a large tool result is processed by RLM.
/// # Examples
/// ```rust,no_run
/// # fn example<B: codetether_rlm_routing::Bus>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, bus: Option<B>) {
/// let context = codetether_rlm_routing::AutoProcessContext {
///     tool_id: "read", tool_args: serde_json::json!({}), session_id: "session",
///     abort: None, on_progress: None, provider, model: "model".into(), bus,
///     trace_id: None, subcall_provider: None, subcall_model: None,
/// };
/// assert_eq!(context.tool_id, "read");
/// # }
/// ```
pub struct AutoProcessContext<'a, B: crate::Bus> {
    /// Originating tool identifier.
    pub tool_id: &'a str,
    /// Original tool input.
    pub tool_args: serde_json::Value,
    /// Owning session identifier.
    pub session_id: &'a str,
    /// Optional cancellation receiver.
    pub abort: Option<tokio::sync::watch::Receiver<bool>>,
    /// Optional progress callback.
    pub on_progress: Option<Box<dyn Fn(ProcessProgress) + Send + Sync>>,
    /// Root provider retained for processing.
    pub provider: Arc<dyn Provider>,
    /// Root provider model.
    pub model: String,
    /// Optional application event bus.
    pub bus: Option<B>,
    /// Optional existing trace identity.
    pub trace_id: Option<Uuid>,
    /// Optional provider override for subcalls.
    pub subcall_provider: Option<Arc<dyn Provider>>,
    /// Optional model override for subcalls.
    pub subcall_model: Option<String>,
}
