//! Request avatar list models.
use super::{AvatarTool, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl AvatarTool {
    pub(super) async fn list_models(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/models");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("List models failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "List models failed ({status}): {body}"
            )));
        }

        super::list_models_result::read(resp).await
    }
}
