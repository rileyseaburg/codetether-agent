//! Validate generate_from_episode arguments and dispatch its request.
use super::{AvatarTool, params::*};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl AvatarTool {
    pub(super) async fn generate_from_episode_args(&self, p: Params) -> Result<ToolResult> {
        let podcast_id =
            match super::required::generate_from_episode_podcast_id::parse(p.podcast_id) {
                Ok(value) => value,
                Err(error) => return Ok(error),
            };
        let episode_id =
            match super::required::generate_from_episode_episode_id::parse(p.episode_id) {
                Ok(value) => value,
                Err(error) => return Ok(error),
            };
        self.generate_from_episode(&EpisodeAvatarParams {
            podcast_id,
            episode_id,
            model_video: p.model_video,
            upload_youtube: p.upload_youtube.unwrap_or(false),
            privacy_status: p.privacy_status,
        })
        .await
    }
}
