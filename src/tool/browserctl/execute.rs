//! Approval and network boundary for browser actions.

use super::{detect_result, dispatch, input, response};
use crate::tool::ToolResult;
use anyhow::{Context, Result, anyhow};
use serde_json::Value;

#[cfg(test)]
#[path = "execute_tests.rs"]
mod tests;

pub(super) async fn run(args: Value) -> Result<ToolResult> {
    let input: input::BrowserCtlInput =
        serde_json::from_value(args.clone()).context("Invalid browserctl args")?;
    if let Some(blocked) =
        crate::runtime_policy::evaluate_tool_invocation("browserctl", &args).await
    {
        return Ok(blocked);
    }
    crate::approval::use_once::claim("browserctl", &args)
        .map_err(|error| anyhow!("approval claim failed: {error}"))?;
    if matches!(&input.action, input::BrowserCtlAction::Detect) {
        return Ok(detect_result());
    }
    let result = match dispatch::dispatch(&input).await {
        Ok(output) => response::success_result(&input, output).await?,
        Err(error) => response::error_result(error),
    };
    if matches!(&input.action, input::BrowserCtlAction::Stop) && result.success {
        crate::browser::browser_service().clear();
    }
    Ok(result)
}
