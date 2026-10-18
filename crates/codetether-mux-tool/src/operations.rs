//! Mux observation and steering operations.

use anyhow::{Context, Result};

use codetether_tool_core::ToolResult;

use super::args::Args;

pub(super) async fn list() -> Result<ToolResult> {
    let value = codetether_mux_control::agent_sessions().await?;
    Ok(ToolResult::success(serde_json::to_string(&value)?))
}

pub(super) async fn read<H: crate::Host>(name: &str) -> Result<ToolResult> {
    result(H::read_agent_output(name).await?, name)
}

pub(super) async fn status(name: &str) -> Result<ToolResult> {
    let value = codetether_mux_control::agent_sessions()
        .await?
        .into_iter()
        .find(|item| codetether_mux_control::is_agent_route(item, name))
        .map(|item| item.to_string());
    result(value, name)
}

pub(super) async fn steer(name: &str, message: &str) -> Result<ToolResult> {
    result(
        codetether_mux_control::send_agent_message(name, message).await?,
        name,
    )
}

pub(super) async fn interact(name: &str) -> Result<ToolResult> {
    result(codetether_mux_control::interact_agent(name).await?, name)
}

pub(super) async fn watch(name: &str, timeout: u64) -> Result<ToolResult> {
    result(
        codetether_mux_control::watch_agent(name, timeout).await?,
        name,
    )
}

pub(super) fn target(args: &Args) -> Result<&str> {
    args.name.as_deref().context("name required")
}

pub(super) fn message(args: &Args) -> Result<&str> {
    args.message.as_deref().context("message required")
}

fn result(value: Option<String>, name: &str) -> Result<ToolResult> {
    Ok(value.map_or_else(
        || ToolResult::error(format!("mux {name} not found")),
        ToolResult::success,
    ))
}
