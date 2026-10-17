//! Fetch the available podcast series.
use super::{PodcastTool, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl PodcastTool {
    pub(super) async fn list_podcasts(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("List podcasts failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "List podcasts failed ({status}): {body}"
            )));
        }

        super::list_podcasts_result::read(resp).await
    }
}
