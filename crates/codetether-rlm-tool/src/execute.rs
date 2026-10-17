//! Validate actions and collect their bounded input.
use crate::{Host, RlmTool, collect, context};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;

pub(crate) async fn run<H: Host>(tool: &RlmTool<H>, args: Value) -> Result<ToolResult> {
    let action = context::action(&args)?;
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

    let paths = context::paths(&args);
    let content = match collect::collect(&paths, args["content"].as_str()).await {
        Ok(content) => content,
        Err(result) => return Ok(result),
    };
    crate::process::process(tool, action, query, &paths, &content).await
}
