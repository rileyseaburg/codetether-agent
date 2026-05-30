//! RLM invocation for session recall results.

mod context;
mod fallback;
mod success;

use crate::provider::Provider;
use crate::rlm::{RlmConfig, RlmRouter};
use crate::tool::ToolResult;
use anyhow::Result;
use std::sync::Arc;

/// Run RLM processing against the flattened recall context.
pub async fn run_recall(
    recall_context: &str,
    sources: &[String],
    query: &str,
    provider: Arc<dyn Provider>,
    model: &str,
    config: &RlmConfig,
) -> Result<ToolResult> {
    let framed = context::frame(query, recall_context);
    let auto_ctx = context::auto(provider, model, query);

    match RlmRouter::auto_process(&framed, auto_ctx, config).await {
        Ok(result) if result.success => Ok(success::format(sources, &result)),
        Ok(result) => Ok(fallback::non_converged(recall_context, sources, &result)),
        Err(error) => Ok(super::faults::fault_result(
            crate::session::Fault::BackendError {
                reason: error.to_string(),
            },
            format!("RLM recall failed: {error}"),
        )),
    }
}
