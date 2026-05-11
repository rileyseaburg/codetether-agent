//! Build the [`AutoProcessContext`] for summary production.

use std::sync::Arc;

use uuid::Uuid;

use crate::provider::Provider;
use crate::rlm::router::AutoProcessContext;
use crate::session::SessionBus;

/// Parameter bundle for building the auto-process context.
pub struct SummaryProduceParams<'a> {
    pub range: super::super::index::types::SummaryRange,
    pub target_tokens: usize,
    pub session_id: &'a str,
    pub provider: Arc<dyn Provider>,
    pub model: &'a str,
    pub subcall_provider: Option<Arc<dyn Provider>>,
    pub subcall_model: Option<&'a str>,
    /// Optional event bus for RLM progress/completion observability.
    pub bus: Option<SessionBus>,
    /// Optional trace id correlating this RLM run with the parent
    /// derivation event.
    pub trace_id: Option<Uuid>,
}

/// Construct the [`AutoProcessContext`] for the RLM call.
pub fn build_auto_context(params: SummaryProduceParams<'_>) -> AutoProcessContext<'_> {
    let SummaryProduceParams {
        range,
        target_tokens,
        session_id,
        provider,
        model,
        subcall_provider,
        subcall_model,
        bus,
        trace_id,
    } = params;
    AutoProcessContext {
        tool_id: "summary_index",
        tool_args: serde_json::json!({
            "range": [range.start, range.end],
            "target_tokens": target_tokens,
        }),
        session_id,
        abort: None,
        on_progress: None,
        provider,
        model: model.to_string(),
        bus,
        trace_id,
        subcall_provider,
        subcall_model: subcall_model.map(|s| s.to_string()),
    }
}
