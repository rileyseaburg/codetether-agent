//! Interpret generate service responses.
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(resp: reqwest::Response) -> Result<ToolResult> {
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!(
            "Avatar generation failed ({status}): {body}"
        )));
    }

    let body: Value = resp.json().await.context("Failed to parse response")?;

    if let Some(error) = body["error"].as_str() {
        return Ok(ToolResult::error(format!(
            "Avatar generation error: {error}"
        )));
    }

    let video_path = body["video_path"].as_str().unwrap_or("unknown");
    let job_code = body["job_code"].as_str().unwrap_or("unknown");

    Ok(ToolResult::success(format!(
        "Avatar video generated!\n\
             Video: {video_path}\n\
             Job: {job_code}"
    ))
    .with_metadata("video_path", json!(video_path))
    .with_metadata("job_code", json!(job_code)))
}
