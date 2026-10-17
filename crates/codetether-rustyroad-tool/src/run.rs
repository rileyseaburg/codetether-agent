//! Approval-aware invocation orchestration and bounded process lifetime.

use super::{Host, invocation, params::Params, process::Process, session};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
use std::time::Duration;

pub(super) async fn execute<H: Host>(input: Value) -> Result<ToolResult> {
    let result = run::<H>(input).await;
    Ok(result.unwrap_or_else(|error| ToolResult::error(format!("RustyRoad: {error:#}"))))
}

async fn run<H: Host>(input: Value) -> Result<ToolResult> {
    let mut params: Params = serde_json::from_value(input.clone())?;
    let cwd = invocation::prepare(&mut params)?;
    if let Some(blocked) =
        H::evaluate_tool_invocation_for_workspace("rustyroad", &input, &cwd).await
    {
        return Ok(blocked);
    }
    let mut process = Process::spawn(&cwd, params.environment.as_str()).await?;
    let result = tokio::time::timeout(
        Duration::from_secs(120),
        session::execute(&mut process.rpc, &params, H::CLIENT_VERSION),
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
