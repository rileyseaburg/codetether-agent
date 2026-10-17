//! Interpret avatar status responses.
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(resp: reqwest::Response) -> Result<ToolResult> {
    let body: Value = resp.json().await.context("Failed to parse response")?;
    let running = body["container_running"].as_bool().unwrap_or(false);
    let image_size = body["image_size"].as_str().unwrap_or("unknown");
    let models = body["models"].as_array().map(|m| m.len()).unwrap_or(0);
    let gpu_sharing = body["gpu_sharing_enabled"].as_bool().unwrap_or(false);

    Ok(ToolResult::success(format!(
        "Duix Avatar Status:\n\
             Container: {}\n\
             Docker Image: {image_size}\n\
             Models available: {models}\n\
             GPU time-sharing: {}",
        if running { "running" } else { "stopped" },
        if gpu_sharing { "enabled" } else { "disabled" },
    ))
    .with_metadata("container_running", json!(running))
    .with_metadata("models_count", json!(models)))
}
