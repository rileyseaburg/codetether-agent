//! Build the validated publish action parameters.
use super::{
    YouTubeTool,
    params::{Params, PublishEpisodeParams},
};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl YouTubeTool {
    pub(super) async fn publish_args(&self, p: Params) -> Result<ToolResult> {
        let podcast_id = match super::publish_fields::podcast_id(p.podcast_id) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        let episode_id = match super::publish_fields::episode_id(p.episode_id) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        self.publish_episode(&PublishEpisodeParams {
            podcast_id,
            episode_id,
            privacy_status: p.privacy_status,
            custom_title: p.custom_title,
            custom_description: p.custom_description,
            tags: p.tags,
        })
        .await
    }
}
