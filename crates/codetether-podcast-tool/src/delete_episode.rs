//! delete episode request and response handling.
use super::{PodcastTool, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl PodcastTool {
    pub(super) async fn delete_episode(
        &self,
        podcast_id: &str,
        episode_id: &str,
    ) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts/{podcast_id}/episodes/{episode_id}");

        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Delete episode failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Delete episode failed ({status}): {body}"
            )));
        }

        Ok(ToolResult::success(format!(
            "Episode {episode_id} deleted from podcast {podcast_id}. RSS feed updated."
        )))
    }
}
