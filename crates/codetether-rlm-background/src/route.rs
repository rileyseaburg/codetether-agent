//! Synchronous routing entry for background RLM.

use std::sync::Arc;

use codetether_provider_core::{Message, Provider};
use codetether_rlm::{RlmConfig, RlmRouter};

use super::{Notify, cache, render, route_ctx};

pub(super) fn route_or_defer(
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
    resolver: crate::Resolver,
) -> String {
    let ctx_window = codetether_provider_core::limits::context_window_for_model(model);
    let context = route_ctx::routing_ctx(tool, call_id, session_id, ctx_window, messages);
    let routing = RlmRouter::should_route(rendered, &context, config);
    if !routing.should_route {
        return rendered.to_string();
    }
    let key = cache::key(tool, input, rendered);
    if let Some(summary) = cache::ready(key) {
        return summary;
    }
    let (truncated, _, _) = RlmRouter::smart_truncate(rendered, tool, input, ctx_window / 4);
    super::route_spawn::start(
        key,
        rendered,
        tool,
        input,
        session_id,
        model,
        provider,
        config,
        notify,
        &routing.reason,
        resolver,
    );
    render::pending(key, tool, rendered.len(), &routing.reason, truncated)
}
