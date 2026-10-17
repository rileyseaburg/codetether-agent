//! Send the generate_from_episode request.
use super::{AvatarTool, params::EpisodeAvatarParams, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl AvatarTool {
    pub(super) async fn generate_from_episode(
        &self,
        params: &EpisodeAvatarParams,
    ) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/generate-from-episode");

        let form = match super::generate_from_episode_form::build(params).await? {
            Ok(form) => form,
            Err(error) => return Ok(error),
        };
        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar episode generation failed: {e}"))?;

        super::generate_from_episode_result::read(resp).await
    }
}
