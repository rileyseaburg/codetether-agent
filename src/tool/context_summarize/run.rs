//! Execution flow for `context_summarize`.

use std::sync::Arc;

use super::super::ToolResult;
use super::super::context_helpers::load_latest_session;
use super::logic::lookup_cached;
use super::tool_struct::ContextSummarizeTool;
use super::{parse, respond};
use anyhow::Result;
use serde_json::Value;

/// Run the summarize tool.
pub async fn run(tool: &ContextSummarizeTool, args: Value) -> Result<ToolResult> {
    let parsed = match parse::parse(&args) {
        Ok(parsed) => parsed,
        Err(message) => return Ok(ToolResult::error(message)),
    };
    let mut session = match load_latest_session().await? {
        Some(s) => s,
        None => return Ok(ToolResult::error("No active session.")),
    };
    if let Some(node) = lookup_cached(&session, parsed.range) {
        return Ok(respond::cached(node));
    }
    let (Some(provider), Some(model)) = (&tool.provider, &tool.model) else {
        return Ok(ToolResult::error(
            "Summary not cached and no provider is available.",
        ));
    };
    let node = super::produce::produce_cached(
        &mut session,
        parsed.range,
        parsed.target,
        Arc::clone(provider),
        model,
    )
    .await?;
    Ok(respond::produced(&node))
}
