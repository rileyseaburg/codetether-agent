//! stop the avatar service container.
use super::{AvatarTool, voice_api_url};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::time::Duration;
impl AvatarTool {
    pub(super) async fn stop(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/stop");

        let resp = self
            .client
            .post(&url)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar stop failed: {e}"))?;

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let message = body["message"].as_str().unwrap_or("Container stopped");

        Ok(ToolResult::success(message))
    }
}
