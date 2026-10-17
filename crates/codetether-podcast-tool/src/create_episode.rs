//! Build and send the create_episode request.
use super::{PodcastTool, params::CreateEpisodeParams, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl PodcastTool {
    pub(super) async fn create_episode(&self, params: &CreateEpisodeParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts/{}/episodes", params.podcast_id);

        let voice_id = params.voice_id.as_deref().unwrap_or("960f89fc");

        let mut form = reqwest::multipart::Form::new()
            .text("title", params.title.clone())
            .text("script", params.script.clone())
            .text("voice_id", voice_id.to_string());

        if let Some(ref desc) = params.description {
            form = form.text("description", desc.clone());
        }
        if let Some(num) = params.episode_number {
            form = form.text("episode_number", num.to_string());
        }
        if let Some(season) = params.season_number {
            form = form.text("season_number", season.to_string());
        }

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Create episode request failed: {e}"))?;

        super::create_episode_result::read(resp, params).await
    }
}
