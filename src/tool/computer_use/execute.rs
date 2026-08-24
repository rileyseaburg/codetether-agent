//! Approval-bound native desktop action dispatch.

use super::{input::ComputerUseInput, platform};
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use serde_json::Value;

pub(super) async fn run(args: Value) -> Result<ToolResult> {
    let input: ComputerUseInput =
        serde_json::from_value(args.clone()).context("Invalid computer_use args")?;
    if let Some(blocked) =
        crate::runtime_policy::evaluate_tool_invocation("computer_use", &args).await
    {
        return Ok(blocked);
    }
    if let Some(blocked) = super::unsafe_process_policy::blocked(&args) {
        return Ok(blocked);
    }
    if let Err(error) = crate::approval::use_once::claim("computer_use", &args) {
        return Ok(ToolResult::error(format!("approval claim failed: {error}")));
    }
    platform::dispatch(&input).await
}
