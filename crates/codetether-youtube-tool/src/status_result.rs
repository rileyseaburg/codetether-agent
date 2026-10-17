//! Format channel statistics and status metadata.
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(resp: reqwest::Response) -> Result<ToolResult> {
    let body: Value = resp.json().await.context("Failed to parse response")?;
    let status = body["status"].as_str().unwrap_or("unknown");
    let channel_title = body["channel_title"].as_str().unwrap_or("unknown");
    let channel_id = body["channel_id"].as_str().unwrap_or("unknown");
    let subs = body["subscriber_count"].as_str().unwrap_or("0");
    let videos = body["video_count"].as_str().unwrap_or("0");

    Ok(ToolResult::success(format!(
        "YouTube API Status: {status}\n\
             Channel: {channel_title}\n\
             Channel ID: {channel_id}\n\
             Subscribers: {subs}\n\
             Videos: {videos}"
    ))
    .with_metadata("channel_id", json!(channel_id))
    .with_metadata("channel_title", json!(channel_title)))
}
