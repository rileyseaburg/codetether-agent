//! Decode undo requests and preserve preview-before-authorization ordering.
use crate::{Policy, UndoInput};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn run<P: Policy>(input: Value) -> Result<ToolResult> {
    let params: UndoInput = serde_json::from_value(input.clone())?;

    // Get current directory
    let cwd = std::env::current_dir()?;

    let (commit_count, commit_list) = match super::history::read(&cwd, params.steps)? {
        Ok(history) => history,
        Err(result) => return Ok(result),
    };
    if params.preview {
        return super::preview::show(&cwd, params.steps, commit_count, &commit_list);
    }
    if let Some(blocked) = P::blocked(&input).await {
        return Ok(blocked);
    }
    let revert_output = super::reset::reset(&cwd, params.steps)?;

    Ok(super::outcome::format(
        revert_output,
        commit_count,
        &commit_list,
    ))
}
