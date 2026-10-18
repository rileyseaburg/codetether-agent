//! Safe mux lifecycle operations for the manager tool.

use anyhow::{Context, Result};
use serde_json::json;

use super::args::Args;
use codetether_tool_core::ToolResult;

pub(super) async fn start(args: &Args) -> Result<ToolResult> {
    let name = target(args)?;
    let workspace = args
        .workspace
        .clone()
        .context("workspace required for start")?;
    let isolation = codetether_mux_workspace::Isolation::from_no_worktree(args.no_worktree);
    let session = codetether_mux_lifecycle::start_managed_session(
        name,
        workspace,
        args.session_id.as_deref(),
        isolation,
    )
    .await?;
    Ok(ToolResult::success(
        json!({"accepted":true,"name":session.name,"status":"idle"}).to_string(),
    ))
}

pub(super) async fn roll(args: &Args) -> Result<ToolResult> {
    super::lifecycle_safety::ensure(args).await?;
    let session = codetether_mux_lifecycle::restart::restart_session(
        target(args)?,
        args.session_id.as_deref(),
    )
    .await?;
    Ok(ToolResult::success(
        json!({"accepted":true,"name":session.name,"status":"idle"}).to_string(),
    ))
}

pub(super) async fn stop(args: &Args) -> Result<ToolResult> {
    super::lifecycle_safety::ensure(args).await?;
    let name = target(args)?;
    codetether_mux_control::stop_session(name).await?;
    Ok(ToolResult::success(
        json!({"accepted":true,"name":name,"status":"stopped"}).to_string(),
    ))
}

fn target(args: &Args) -> Result<&str> {
    args.name.as_deref().context("name required")
}
