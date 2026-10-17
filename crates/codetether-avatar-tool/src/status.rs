//! Request avatar status.
use super::{AvatarTool, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use std::time::Duration;
impl AvatarTool {
    pub(super) async fn status(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/status");

        let resp = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar status check failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Avatar status failed ({status}): {body}"
            )));
        }

        super::status_result::read(resp).await
    }
}
