//! Build multipart fields for upload_model.
use super::params::UploadModelParams;
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
pub(super) async fn build(
    params: &UploadModelParams,
) -> Result<Result<reqwest::multipart::Form, ToolResult>> {
    let file_path = std::path::Path::new(&params.file_path);
    if !file_path.exists() {
        return Ok(Err(ToolResult::error(format!(
            "Video file not found: {}",
            params.file_path
        ))));
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

    let form = reqwest::multipart::Form::new()
        .text("name", params.name.clone())
        .part("video", part);

    Ok(Ok(form))
}
