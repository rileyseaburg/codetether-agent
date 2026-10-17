//! Validate get_podcast arguments and dispatch its request.
use super::{PodcastTool, params::*};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl PodcastTool {
    pub(super) async fn get_podcast_args(&self, p: Params) -> Result<ToolResult> {
        let podcast_id = match super::required::get_podcast_podcast_id::parse(p.podcast_id) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        self.get_podcast(&podcast_id).await
    }
}
