//! Preserve the recall synthesis context and fresh trace identity.
use codetether_provider_core::Provider;
use codetether_rlm_routing::{AutoProcessContext, Bus};
use serde_json::json;
use std::sync::Arc;
pub(super) fn auto<'a, B: Bus>(
    provider: Arc<dyn Provider>,
    model: &str,
    query: &'a str,
) -> AutoProcessContext<'a, B> {
    AutoProcessContext {
        tool_id: "session_recall",
        tool_args: json!({ "query": query }),
        session_id: "session-recall-tool",
        abort: None,
        on_progress: None,
        provider,
        model: model.to_string(),
        bus: None,
        trace_id: Some(uuid::Uuid::new_v4()),
        subcall_provider: None,
        subcall_model: None,
    }
}
