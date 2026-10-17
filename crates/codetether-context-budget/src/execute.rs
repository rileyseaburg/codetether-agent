//! Load only the configured working window before reporting its budget.
use crate::BudgetSource;
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn run<S: BudgetSource>(args: Value) -> Result<ToolResult> {
    let session = match codetether_context_loader::load_calling_window::<S>(&args, S::WINDOW).await
    {
        Ok(Some(s)) => s,
        Ok(None) => return Ok(ToolResult::error("No active session found.")),
        Err(e) => return Ok(ToolResult::error(&format!("Failed to load session: {e}"))),
    };
    Ok(super::render::budget(
        session.messages_len(),
        session.cached_summaries(),
        session.pages(),
    ))
}
