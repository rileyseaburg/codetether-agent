//! Validate create_episode arguments and dispatch its request.
use super::{PodcastTool, params::*};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl PodcastTool {
    pub(super) async fn create_episode_args(&self, p: Params) -> Result<ToolResult> {
        let podcast_id = match super::required::create_episode_podcast_id::parse(p.podcast_id) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        let title = match super::required::create_episode_title::parse(p.title) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        let script = match super::required::create_episode_script::parse(p.script) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        self.create_episode(&CreateEpisodeParams {
            podcast_id,
            title,
            script,
            voice_id: p.voice_id,
            description: p.description,
            episode_number: p.episode_number,
            season_number: p.season_number,
        })
        .await
    }
}
