//! Build the [`AutoProcessContext`] for summary production.

use std::sync::Arc;

use crate::provider::Provider;
use crate::rlm::router::AutoProcessContext;

/// Parameter bundle for building the auto-process context.
pub struct SummaryProduceParams<'a> {
    pub range: super::super::index::types::SummaryRange,
    pub target_tokens: usize,
    pub session_id: &'a str,
    pub provider: Arc<dyn Provider>,
    pub model: &'a str,
    pub subcall_provider: Option<Arc<dyn Provider>>,
    pub subcall_model: Option<&'a str>,
}

/// Construct the [`AutoProcessContext`] for the RLM call.
pub fn build_auto_context(params: SummaryProduceParams<'_>) -> AutoProcessContext<'_> {
    use super::super::index::types::SummaryRange;
    let SummaryProduceParams {
        range,
        target_tokens,
        session_id,
        provider,
        model,
        subcall_provider,
        subcall_model,
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
        bus: None,
        trace_id: None,
        subcall_provider,
        subcall_model: subcall_model.map(|s| s.to_string()),
    }
}
