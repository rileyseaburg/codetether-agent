//! Preserve the application's RLM session context and trace construction.
use crate::provider::Provider;
use crate::rlm::router::AutoProcessContext;
use serde_json::Value;
use std::sync::Arc;
pub(super) fn auto<'a>(
    provider: Arc<dyn Provider>,
    model: &str,
    action: &'a str,
    tool_args: Value,
) -> AutoProcessContext<'a> {
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
