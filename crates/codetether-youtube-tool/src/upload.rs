//! Read the local video and submit its upload request.
use super::{YouTubeTool, params::UploadVideoParams, voice_api_url};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
impl YouTubeTool {
    pub(super) async fn upload_video(&self, params: &UploadVideoParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/youtube/upload");

        let file_path = std::path::Path::new(&params.file_path);
        if !file_path.exists() {
            return Ok(ToolResult::error(format!(
                "Video file not found: {}",
                params.file_path
            )));
        }

        let file_bytes = tokio::fs::read(file_path)
            .await
            .context("Failed to read video file")?;

        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("video/mp4")?;

        let form = super::upload_form::build(part, params);
        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("YouTube upload request failed: {e}"))?;

        super::upload_result::read(resp).await
    }
}
