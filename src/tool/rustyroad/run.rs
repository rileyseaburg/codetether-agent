//! Approval-aware invocation orchestration and bounded process lifetime.

use super::{invocation, params::Params, process::Process, session};
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::time::Duration;

pub(super) async fn execute(input: Value) -> Result<ToolResult> {
    let result = run(input).await;
    Ok(result.unwrap_or_else(|error| ToolResult::error(format!("RustyRoad: {error:#}"))))
}

async fn run(input: Value) -> Result<ToolResult> {
    let mut params: Params = serde_json::from_value(input.clone())?;
    let cwd = invocation::prepare(&mut params)?;
    if let Some(blocked) =
        crate::runtime_policy::evaluate_tool_invocation_for_workspace("rustyroad", &input, &cwd)
            .await
    {
        return Ok(blocked);
    }
    let mut process = Process::spawn(&cwd, params.environment.as_str()).await?;
    let result = tokio::time::timeout(
        Duration::from_secs(120),
        session::execute(&mut process.rpc, &params),
    )
    .await
    .context("RustyRoad timed out after 120 seconds")
    .and_then(std::convert::identity);
    let cleanup = process.close().await;
    let result = result?;
    cleanup?;
    Ok(result
        .with_metadata("rustyroad_project", json!(cwd))
        .with_metadata("rustyroad_environment", json!(params.environment.as_str())))
}
