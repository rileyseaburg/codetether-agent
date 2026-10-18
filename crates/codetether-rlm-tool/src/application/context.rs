//! Preserve RLM session context and per-request trace identity.
use codetether_provider_core::Provider;
use codetether_rlm_routing::{AutoProcessContext, Bus};
use serde_json::Value;
use std::sync::Arc;
pub(super) fn auto<'a, B: Bus>(
    provider: Arc<dyn Provider>,
    model: &str,
    action: &'a str,
    tool_args: Value,
) -> AutoProcessContext<'a, B> {
    AutoProcessContext {
        tool_id: action,
        tool_args,
        session_id: "rlm-tool",
        abort: None,
        on_progress: None,
        provider,
        model: model.to_owned(),
        bus: None,
        trace_id: Some(uuid::Uuid::new_v4()),
        subcall_provider: None,
        subcall_model: None,
    }
}
