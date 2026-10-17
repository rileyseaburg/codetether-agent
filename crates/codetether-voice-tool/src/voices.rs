//! Fetch configured voice profiles.
use super::{VoiceTool, default_voice_api_url};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::Value;
impl VoiceTool {
    pub(super) async fn list_voices(&self) -> Result<ToolResult> {
        let base_url = default_voice_api_url();
        let url = format!("{base_url}/voices");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Voice API request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Voice API returned {status}: {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;

        Ok(super::voices_result::format(body))
    }
}
