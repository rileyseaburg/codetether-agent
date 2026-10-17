//! Validate the summary, record acceptance, then return the reset marker.
use crate::{audit, format_reset_marker};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn run(args: Value) -> Result<ToolResult> {
    let summary = match args["summary"].as_str() {
        Some(s) if !s.trim().is_empty() => s.trim().to_string(),
        _ => {
            return Ok(ToolResult::error(
                "`summary` is required and must be non-empty",
            ));
        }
    };
    let bytes = summary.len();
    audit::log(bytes).await;
    Ok(ToolResult::success(format_reset_marker(&summary, bytes)))
}
