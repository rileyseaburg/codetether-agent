//! Interpret upload_model service responses.
use super::params::UploadModelParams;
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(
    resp: reqwest::Response,
    params: &UploadModelParams,
) -> Result<ToolResult> {
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!(
            "Upload model failed ({status}): {body}"
        )));
    }

    let body: Value = resp.json().await.context("Failed to parse response")?;
    let model_path = body["model_path"].as_str().unwrap_or("unknown");
    let size = body["size_bytes"].as_u64().unwrap_or(0);
    let size_mb = size as f64 / 1_048_576.0;

    Ok(ToolResult::success(format!(
        "Model uploaded!\n\
             Name: {}\n\
             Path: {model_path}\n\
             Size: {size_mb:.1}MB\n\n\
             You can now use this model for avatar video generation.",
        params.name
    ))
    .with_metadata("model_path", json!(model_path)))
}
