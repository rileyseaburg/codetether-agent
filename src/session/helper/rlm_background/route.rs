//! Synchronous routing entry for background RLM.

use std::sync::Arc;

use crate::provider::{Message, Provider};
use crate::rlm::{RlmConfig, RlmRouter};

use super::{Notify, cache, job, render, route_ctx, thread};

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
) -> String {
    let ctx_window = super::super::token::context_window_for_model(model);
    let routing = RlmRouter::should_route(
        rendered,
        &route_ctx::routing_ctx(tool, call_id, session_id, ctx_window, messages),
        config,
    );
    if !routing.should_route {
        return rendered.to_string();
    }
    let key = cache::key(tool, input, rendered);
    if let Some(summary) = cache::ready(key) {
        return summary;
    }
    let (truncated, _, _) = RlmRouter::smart_truncate(rendered, tool, input, ctx_window / 4);
    if cache::claim(key) {
        thread::spawn(job::Job {
            key,
            content: rendered.into(),
            tool: tool.into(),
            input: input.clone(),
            session_id: session_id.into(),
            model: model.into(),
            provider,
            config: config.clone(),
            reason: routing.reason.clone(),
            original_bytes: rendered.len(),
            notify,
        });
    }
    render::pending(key, tool, rendered.len(), &routing.reason, truncated)
}
