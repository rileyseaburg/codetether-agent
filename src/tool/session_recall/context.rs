//! Preserve the application RLM context and trace identity for answer synthesis.
use crate::provider::Provider;
use crate::rlm::router::AutoProcessContext;
use serde_json::json;
use std::sync::Arc;

pub(super) fn auto<'a>(
    provider: Arc<dyn Provider>,
    model: &str,
    query: &'a str,
) -> AutoProcessContext<'a> {
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
