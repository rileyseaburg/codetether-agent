//! Attach an optional local WAV file to a generation form.
use super::params::GenerateParams;
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
pub(super) async fn attach(
    mut form: reqwest::multipart::Form,
    params: &GenerateParams,
) -> Result<Result<reqwest::multipart::Form, ToolResult>> {
    if let Some(ref audio_file) = params.audio_file {
        let file_path = std::path::Path::new(audio_file);
        if !file_path.exists() {
            return Ok(Err(ToolResult::error(format!(
                "Audio file not found: {audio_file}"
            ))));
        }
        let file_bytes = tokio::fs::read(file_path)
            .await
            .context("Failed to read audio file")?;
        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("audio/wav")?;
        form = form.part("audio_file", part);
    }
    Ok(Ok(form))
}
