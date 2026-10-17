//! Build and send the episode publishing request.
use super::{YouTubeTool, params::PublishEpisodeParams, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl YouTubeTool {
    pub(super) async fn publish_episode(
        &self,
        params: &PublishEpisodeParams,
    ) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/youtube/publish-episode");

        let privacy = params.privacy_status.as_deref().unwrap_or("unlisted");

        let mut form = reqwest::multipart::Form::new()
            .text("podcast_id", params.podcast_id.clone())
            .text("episode_id", params.episode_id.clone())
            .text("privacy_status", privacy.to_string());

        if let Some(ref title) = params.custom_title {
            form = form.text("custom_title", title.clone());
        }
        if let Some(ref desc) = params.custom_description {
            form = form.text("custom_description", desc.clone());
        }
        if let Some(ref tags) = params.tags {
            form = form.text("tags", tags.clone());
        }

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("YouTube publish request failed: {e}"))?;

        super::publish_result::read(resp).await
    }
}
