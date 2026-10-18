//! Public route_or_defer entry point.
use crate::{Notify, Resolver};
use codetether_provider_core::{Message, Provider};
use codetether_rlm::RlmConfig;
use std::sync::Arc;

/// Return a cached summary, raw output, or bounded fallback while RLM runs.
/// Return raw output, a ready summary, or a bounded placeholder while a job runs.
///
/// The resolver runs inside the worker and retains application model policy.
/// Cache keys and completed results are shared across calls in the process.
/// # Arguments
/// * `rendered` — Complete tool output before routing.
/// * `tool` — Tool name for cache keys and traces.
/// * `input` — Original tool arguments.
/// * `call_id` — Tool call identifier.
/// * `session_id` — Owning session identifier.
/// * `messages` — Current transcript used in routing estimates.
/// * `model` — Caller model selector.
/// * `provider` — Caller provider.
/// * `config` — Routing and RLM settings.
/// * `notify` — Optional progress callback.
/// * `resolver` — Native model-resolution callback.
/// # Returns
/// Original output when routing is unnecessary, a ready summary, or a bounded placeholder.
pub fn route_or_defer(
    rendered: &str,
    tool: &str,
    input: &serde_json::Value,
    call_id: &str,
    session_id: &str,
    messages: &[Message],
    model: &str,
    provider: Arc<dyn Provider>,
    config: &RlmConfig,
    notify: Option<Notify>,
    resolver: Resolver,
) -> String {
    crate::route::route_or_defer(
        rendered, tool, input, call_id, session_id, messages, model, provider, config, notify,
        resolver,
    )
}
