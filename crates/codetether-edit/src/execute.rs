//! Read content and prepare confirmation previews without writing.

use anyhow::Result;
use serde_json::Value;
use tokio::fs;

use super::args::{EditArgs, required};
use super::diff;
use super::matcher::MatchPlan;
use super::metadata;
use super::morph_flow;
use codetether_tool_core::ToolResult;

pub async fn run(args: Value, editor: &dyn codetether_morph_backend::Editor) -> Result<ToolResult> {
    let args = match EditArgs::parse(&args) {
        Ok(parsed) => parsed,
        Err(error) => return Ok(error),
    };
    if let Some(blocked) = codetether_tool_core::temp_write_guard::denied_result("edit", args.path)
    {
        return Ok(blocked);
    }
    let content = fs::read_to_string(args.path).await?;
    if let Some(result) = morph_flow::try_apply(&args, &content, editor).await {
        return Ok(result);
    }
    Ok(preview_replacement(&args, &content))
}

fn preview_replacement(args: &EditArgs<'_>, content: &str) -> ToolResult {
    let old_string = match required(args.old_string, "old_string", args.path) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let new_string = match required(args.new_string, "new_string", args.path) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let plan = match MatchPlan::find(content, old_string, args.replace_all) {
        Ok(plan) => plan,
        Err(error) => return error,
    };
    let new_content = plan.apply(content, new_string);
    let preview = diff::preview(content, &new_content);
    metadata::confirmation(args.path, old_string, new_string, &plan, preview)
}
