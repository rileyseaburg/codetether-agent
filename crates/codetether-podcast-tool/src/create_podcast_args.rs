//! Validate create_podcast arguments and dispatch its request.
use super::{PodcastTool, params::*};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl PodcastTool {
    pub(super) async fn create_podcast_args(&self, p: Params) -> Result<ToolResult> {
        let title = match super::required::create_podcast_title::parse(p.title) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        let description = match super::required::create_podcast_description::parse(p.description) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        self.create_podcast(&CreatePodcastParams {
            title,
            description,
            author: p.author,
            email: p.email,
            category: p.category,
            subcategory: p.subcategory,
            language: p.language,
        })
        .await
    }
}
