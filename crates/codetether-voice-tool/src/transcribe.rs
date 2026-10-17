//! Read an audio file and submit its multipart transcription request.
use super::{VoiceTool, default_voice_api_url, params::TranscribeParams};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
impl VoiceTool {
    pub(super) async fn transcribe(&self, params: &TranscribeParams) -> Result<ToolResult> {
        let base_url = default_voice_api_url();
        let url = format!("{base_url}/transcribe");

        let file_path = std::path::Path::new(&params.file_path);
        if !file_path.exists() {
            return Ok(ToolResult::error(format!(
                "File not found: {}",
                params.file_path
            )));
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
            .mime_str("application/octet-stream")?;

        let form = reqwest::multipart::Form::new().part("audio_file", part);

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Transcription request failed: {e}"))?;

        super::transcribe_result::read(resp, params).await
    }
}
