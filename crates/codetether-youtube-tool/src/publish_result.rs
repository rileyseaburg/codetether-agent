//! Decode publishing responses and video metadata.
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(resp: reqwest::Response) -> Result<ToolResult> {
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!(
            "YouTube publish failed ({status}): {body}"
        )));
    }

    let body: Value = resp.json().await.context("Failed to parse response")?;
    let video_id = body["video_id"].as_str().unwrap_or("unknown");
    let video_url = body["url"].as_str().unwrap_or("unknown");
    let title = body["title"].as_str().unwrap_or("unknown");
    let channel = body["channel_title"].as_str().unwrap_or("unknown");

    Ok(ToolResult::success(format!(
        "Published to YouTube!\n\
             Video: {video_url}\n\
             Title: {title}\n\
             Channel: {channel}\n\
             Video ID: {video_id}"
    ))
    .with_metadata("video_id", json!(video_id))
    .with_metadata("url", json!(video_url))
    .with_metadata("title", json!(title)))
}
