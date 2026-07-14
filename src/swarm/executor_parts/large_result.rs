//! RLM summarization for oversized tool output.

use crate::provider::Provider;
use crate::rlm::RlmExecutor;
use crate::swarm::token_truncate::truncate_single_result;
use std::sync::Arc;

pub(super) const RLM_THRESHOLD_CHARS: usize = 50_000;
pub(super) const SIMPLE_TRUNCATE_CHARS: usize = 6_000;

pub(super) async fn process(
    content: &str,
    tool_name: &str,
    provider: Arc<dyn Provider>,
    model: &str,
) -> String {
    if content.len() <= SIMPLE_TRUNCATE_CHARS {
        return content.to_string();
    }
    if content.len() <= RLM_THRESHOLD_CHARS {
        return truncate_single_result(content, SIMPLE_TRUNCATE_CHARS);
    }
    tracing::info!(tool = %tool_name, content_len = content.len(), "Summarizing large tool result");
    summarize(content, tool_name, provider, model).await
}

async fn summarize(content: &str, tool: &str, provider: Arc<dyn Provider>, model: &str) -> String {
    let query =
        format!("Summarize key errors, warnings, findings, and actions from this {tool} output.");
    let mut rlm =
        RlmExecutor::new(content.to_string(), provider, model.to_string()).with_max_iterations(3);
    match rlm.analyze(&query).await {
        Ok(result) => super::large_result_format::success(content, tool, &result.answer),
        Err(error) => {
            tracing::warn!(tool = %tool, %error, "RLM analysis failed; truncating result");
            truncate_single_result(content, SIMPLE_TRUNCATE_CHARS)
        }
    }
}
