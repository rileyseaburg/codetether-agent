//! Execution flow for the RLM tool.

use super::{RlmTool, ToolResult, collect, ctx, process};
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
    crate::tool::network_access::guard!("rlm", &args);

    let paths = ctx::paths(&args);
    let content = match collect::collect(&paths, args["content"].as_str()).await {
        Ok(content) => content,
        Err(result) => return Ok(result),
    };
    process::run(tool, action, query, &paths, &content).await
}

#[cfg(test)]
#[path = "approval_tests.rs"]
mod tests;
