//! Validate delete_episode arguments and dispatch its request.
use super::{PodcastTool, params::*};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl PodcastTool {
    pub(super) async fn delete_episode_args(&self, p: Params) -> Result<ToolResult> {
        let podcast_id = match super::required::delete_episode_podcast_id::parse(p.podcast_id) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        let episode_id = match super::required::delete_episode_episode_id::parse(p.episode_id) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        self.delete_episode(&podcast_id, &episode_id).await
    }
}
