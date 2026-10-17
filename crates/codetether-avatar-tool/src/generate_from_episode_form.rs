//! Build multipart fields for generate_from_episode.
use super::params::EpisodeAvatarParams;
use anyhow::Result;
use codetether_tool_core::ToolResult;
pub(super) async fn build(
    params: &EpisodeAvatarParams,
) -> Result<Result<reqwest::multipart::Form, ToolResult>> {
    let mut form = reqwest::multipart::Form::new()
        .text("podcast_id", params.podcast_id.clone())
        .text("episode_id", params.episode_id.clone());

    if let Some(ref model) = params.model_video {
        form = form.text("model_video", model.clone());
    }
    if params.upload_youtube {
        form = form.text("upload_youtube", "true".to_string());
    }
    if let Some(ref privacy) = params.privacy_status {
        form = form.text("privacy_status", privacy.clone());
    }

    Ok(Ok(form))
}
