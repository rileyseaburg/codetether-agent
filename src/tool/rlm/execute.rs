//! Execution flow for the RLM tool.

use super::{RlmTool, ToolResult, collect, ctx, fallback};
use crate::rlm::RlmRouter;
use anyhow::Result;
use serde_json::Value;

pub(super) async fn run(tool: &RlmTool, args: Value) -> Result<ToolResult> {
    let action = ctx::action(&args)?;
    if !matches!(action, "analyze" | "summarize" | "search") {
        return Ok(ToolResult::error(format!(
            "Unknown action: {action}. Use 'analyze', 'summarize', or 'search'."
        )));
    }

    let query = args["query"].as_str().unwrap_or("");
    if action != "summarize" && query.is_empty() {
        return Ok(ToolResult::error(format!(
            "query is required for '{action}' action"
        )));
    }

    let paths = ctx::paths(&args);
    let content = match collect::collect(&paths, args["content"].as_str()).await {
        Ok(content) => content,
        Err(result) => return Ok(result),
    };
    process(tool, action, query, &paths, &content).await
}

async fn process(
    tool: &RlmTool,
    action: &str,
    query: &str,
    paths: &[&str],
    content: &str,
) -> Result<ToolResult> {
    match RlmRouter::auto_process(content, ctx::auto(tool, action, query, paths), &tool.config).await {
        Ok(result) if result.success => Ok(ToolResult::success(format!(
            "RLM {action} complete ({} → {} tokens, {} iterations)\n\n{}",
            result.stats.input_tokens,
            result.stats.output_tokens,
            result.stats.iterations,
            result.processed
        ))),
        Ok(result) => {
            tracing::warn!(input_tokens = result.stats.input_tokens, output_tokens = result.stats.output_tokens, "RLM auto_process did not converge for tool invocation");
            Ok(fallback::non_converged(action, content))
        }
        Err(error) => {
            tracing::warn!(%error, "RLM auto_process failed, falling back to truncation");
            Ok(fallback::failed(action, content, error))
        }
    }
}
