//! get podcast request and response handling.
use super::{PodcastTool, voice_api_url};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::Value;
impl PodcastTool {
    pub(super) async fn get_podcast(&self, podcast_id: &str) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts/{podcast_id}");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Get podcast failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Get podcast failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let formatted = serde_json::to_string_pretty(&body).unwrap_or_else(|_| body.to_string());

        Ok(ToolResult::success(formatted))
    }
}
