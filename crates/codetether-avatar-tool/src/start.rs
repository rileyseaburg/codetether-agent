//! start the avatar service container.
use super::{AvatarTool, voice_api_url};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::time::Duration;
impl AvatarTool {
    pub(super) async fn start(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/start");

        let resp = self
            .client
            .post(&url)
            .timeout(Duration::from_secs(120))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar start failed: {e}"))?;

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let status = body["status"].as_str().unwrap_or("unknown");
        let message = body["message"].as_str().unwrap_or("");

        if status == "started" {
            Ok(ToolResult::success(format!(
                "Duix Avatar container started. {message}"
            )))
        } else {
            Ok(ToolResult::error(format!(
                "Failed to start avatar container: {message}"
            )))
        }
    }
}
