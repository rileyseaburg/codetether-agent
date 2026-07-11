//! Foreground RLM summarisation for budget-critical context compaction.

use std::sync::Arc;

use crate::provider::Provider;
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmChunker, RlmConfig, RlmRouter};

#[path = "rlm_local_model.rs"]
mod local_model;

pub(super) async fn context_summary(
    context: &str,
    ctx_window: usize,
    session_id: &str,
    reason: &str,
    model: &str,
    provider: Arc<dyn Provider>,
    config: &RlmConfig,
) -> Option<String> {
    if RlmChunker::estimate_tokens(context) == 0 {
        return None;
    }
    let (provider, model) = local_model::resolve(provider, model).await;
    let request = AutoProcessContext {
        tool_id: "session_context",
        tool_args: serde_json::json!({ "reason": reason }),
        session_id,
        abort: None,
        on_progress: None,
        provider,
        model,
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
    };
    match RlmRouter::auto_process(context, request, config).await {
        Ok(result) => Some(result.processed),
        Err(error) => {
            tracing::warn!(%error, "Foreground RLM compaction failed; using deterministic compression");
            Some(RlmChunker::compress(context, ctx_window / 4, None))
        }
    }
}
