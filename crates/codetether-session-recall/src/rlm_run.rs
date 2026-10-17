//! RLM invocation for session recall results.

mod context;
mod fallback;
mod success;

use anyhow::Result;
use codetether_provider_core::Provider;
use codetether_rlm::RlmConfig;
use codetether_tool_core::ToolResult;
use std::sync::Arc;

/// Run RLM processing against the flattened recall context.
pub async fn run_recall<H: crate::Host>(
    recall_context: &str,
    sources: &[String],
    query: &str,
    provider: Arc<dyn Provider>,
    model: &str,
    config: &RlmConfig,
) -> Result<ToolResult> {
    let framed = context::frame(query, recall_context);

    match H::synthesize(&framed, query, provider, model, config).await {
        Ok(result) if result.success => Ok(success::format(sources, &result)),
        Ok(result) => Ok(fallback::non_converged(recall_context, sources, &result)),
        Err(error) => Ok(super::faults::fault_result(
            codetether_context_policy::fault::Fault::BackendError {
                reason: error.to_string(),
            },
            format!("RLM recall failed: {error}"),
        )),
    }
}
