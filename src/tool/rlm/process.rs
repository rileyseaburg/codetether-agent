//! Provider-backed RLM processing after network authorization.

use super::{RlmTool, ToolResult, ctx, fallback};
use crate::rlm::RlmRouter;
use anyhow::Result;

pub(super) async fn run(
    tool: &RlmTool,
    action: &str,
    query: &str,
    paths: &[&str],
    content: &str,
) -> Result<ToolResult> {
    match RlmRouter::auto_process(content, ctx::auto(tool, action, query, paths), &tool.config)
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
            tracing::warn!(
                input_tokens = result.stats.input_tokens,
                output_tokens = result.stats.output_tokens,
                "RLM auto_process did not converge for tool invocation"
            );
            Ok(fallback::non_converged(action, content))
        }
        Err(error) => {
            tracing::warn!(%error, "RLM auto_process failed, falling back to truncation");
            Ok(fallback::failed(action, content, error))
        }
    }
}
