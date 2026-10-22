//! Execution flow for `context_summarize`.

use std::sync::Arc;

use codetether_context_loader::load_calling_session;
use codetether_tool_core::ToolResult;

use super::tool_struct::ContextSummarizeTool;
use super::{parse, respond};
use anyhow::Result;
use serde_json::Value;

/// Run the summarize tool.
pub async fn run<S: crate::SummarySource>(
    tool: &ContextSummarizeTool<S>,
    args: Value,
) -> Result<ToolResult> {
    let parsed = match parse::parse(&args) {
        Ok(parsed) => parsed,
        Err(message) => return Ok(ToolResult::error(message)),
    };
    let mut session = match load_calling_session::<S>(&args).await? {
        Some(s) => s,
        None => return Ok(ToolResult::error("No active session.")),
    };
    super::produce::validate_cache(&mut session);
    if let Some(node) = session.summary_index().get(parsed.range) {
        return Ok(respond::cached(node));
    }
    let (Some(provider), Some(model)) = (&tool.provider, &tool.model) else {
        return Ok(ToolResult::error(
            "Summary not cached and no provider is available.",
        ));
    };
    let node = super::produce::produce_cached::<S>(
        &mut session,
        parsed.range,
        parsed.target,
        Arc::clone(provider),
        model,
    )
    .await?;
    Ok(respond::produced(&node))
}
