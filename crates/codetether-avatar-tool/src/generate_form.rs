//! Build multipart fields for generate.
use super::params::GenerateParams;
use anyhow::Result;
use codetether_tool_core::ToolResult;
pub(super) async fn build(
    params: &GenerateParams,
) -> Result<Result<reqwest::multipart::Form, ToolResult>> {
    let mut form = reqwest::multipart::Form::new();

    if let Some(ref text) = params.text {
        form = form.text("text", text.clone());
    }
    if let Some(ref audio_url) = params.audio_url {
        form = form.text("audio_url", audio_url.clone());
    }
    form = match super::generate_audio::attach(form, params).await? {
        Ok(form) => form,
        Err(error) => return Ok(Err(error)),
    };
    if let Some(ref model) = params.model_video {
        form = form.text("model_video", model.clone());
    }
    if let Some(ref voice_id) = params.voice_id {
        form = form.text("voice_id", voice_id.clone());
    }

    Ok(Ok(form))
}
