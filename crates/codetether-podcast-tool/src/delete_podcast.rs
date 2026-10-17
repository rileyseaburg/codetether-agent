//! delete podcast request and response handling.
use super::{PodcastTool, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl PodcastTool {
    pub(super) async fn delete_podcast(&self, podcast_id: &str) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts/{podcast_id}");

        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Delete podcast failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Delete podcast failed ({status}): {body}"
            )));
        }

        Ok(ToolResult::success(format!(
            "Podcast {podcast_id} and all its episodes have been deleted."
        )))
    }
}
