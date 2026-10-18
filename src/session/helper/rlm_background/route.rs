//! Native route_or_defer adapter.
use super::{Notify, host};
use crate::provider::{Message, Provider};
use crate::rlm::RlmConfig;
use std::sync::Arc;

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
    codetether_rlm_background::route_or_defer(
        rendered,
        tool,
        input,
        call_id,
        session_id,
        messages,
        model,
        provider,
        config,
        host::notify(notify),
        host::resolve,
    )
}
