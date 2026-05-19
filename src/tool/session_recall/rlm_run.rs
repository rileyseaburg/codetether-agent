//! RLM invocation for session recall results.

use crate::provider::Provider;
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmConfig, RlmRouter};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::json;
use std::sync::Arc;

/// Run RLM processing against the flattened recall context.
pub async fn run_recall(
    context: &str,
    sources: &[String],
    query: &str,
    provider: Arc<dyn Provider>,
    model: &str,
    config: &RlmConfig,
) -> Result<ToolResult> {
    let auto_ctx = AutoProcessContext {
        tool_id: "session_recall",
        tool_args: json!({ "query": query }),
        session_id: "session-recall-tool",
        abort: None,
        on_progress: None,
        provider,
        model: model.to_string(),
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
    };
    let framed = format!(
        "Recall task: {query}\n\nUse the session transcript below \
         to answer the recall task. Quote short passages verbatim \
         when useful; otherwise summarise.\n\n{context}"
    );
    match RlmRouter::auto_process(&framed, auto_ctx, config).await {
        Ok(result) => Ok(ToolResult::success(format!(
            "Recalled from {} session(s): {}\n(RLM: {} → {} tokens, {} iterations)\n\n{}",
            sources.len(),
            sources.join(", "),
            result.stats.input_tokens,
            result.stats.output_tokens,
            result.stats.iterations,
            result.processed,
        ))),
        Err(e) => Ok(super::faults::fault_result(
            crate::session::Fault::BackendError {
                reason: e.to_string(),
            },
            format!("RLM recall failed: {e}"),
        )),
    }
}
