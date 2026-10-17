//! Forward content to the router and render success or fallback output.
use crate::{Host, RlmTool, fallback};
use anyhow::Result;
use codetether_tool_core::ToolResult;

pub(crate) async fn process<H: Host>(
    tool: &RlmTool<H>,
    action: &str,
    query: &str,
    paths: &[&str],
    content: &str,
) -> Result<ToolResult> {
    match H::process(
        content,
        action,
        crate::context::args(query, paths),
        std::sync::Arc::clone(&tool.provider),
        &tool.model,
        &tool.config,
    )
    .await
    {
        Ok(result) if result.success => Ok(ToolResult::success(format!(
            "RLM {action} complete ({} → {} tokens, {} iterations)\n\n{}",
            result.stats.input_tokens,
            result.stats.output_tokens,
            result.stats.iterations,
            result.processed
        ))),
        Ok(result) => {
            tracing::warn!(target: "codetether_agent::tool::rlm::execute",
                input_tokens = result.stats.input_tokens,
                output_tokens = result.stats.output_tokens,
                "RLM auto_process did not converge for tool invocation"
            );
            Ok(fallback::non_converged(action, content))
        }
        Err(error) => {
            tracing::warn!(target: "codetether_agent::tool::rlm::execute",%error, "RLM auto_process failed, falling back to truncation");
            Ok(fallback::failed(action, content, error))
        }
    }
}
