//! Background RLM summarisation for large tool outputs.

mod cache;
mod context;
mod disk;
mod gate;
mod job;
mod key;
mod render;
mod resolve;
mod route;
mod route_ctx;
mod status;
mod summary;
mod thread;

use std::sync::Arc;

use crate::provider::{Message, Provider};
use crate::rlm::RlmConfig;

pub(crate) type Notify = status::Notify;

pub(crate) fn context_summary(
    content: &str,
    reason: &str,
    session_id: &str,
    model: &str,
    provider: Arc<dyn Provider>,
    config: &RlmConfig,
) -> Option<String> {
    context::summary_or_spawn(content, reason, session_id, model, provider, config)
}

/// Return a cached summary, raw output, or bounded fallback while RLM runs.
pub(crate) fn route_or_defer(
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
    route::route_or_defer(
        rendered, tool, input, call_id, session_id, messages, model, provider, config, notify,
    )
}

/// Replace pending background placeholders with ready cached summaries.
pub(super) fn resolve_pending(messages: &mut [Message]) {
    resolve::messages(messages)
}
