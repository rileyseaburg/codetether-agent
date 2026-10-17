//! Decode create_podcast responses and result metadata.
use super::params::CreatePodcastParams;
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(
    resp: reqwest::Response,
    params: &CreatePodcastParams,
) -> Result<ToolResult> {
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!(
            "Create podcast failed ({status}): {body}"
        )));
    }

    let body: Value = resp.json().await.context("Failed to parse response")?;
    let podcast_id = body["podcast_id"].as_str().unwrap_or("unknown");
    let feed_url = body["feed_url"].as_str().unwrap_or("unknown");

    Ok(ToolResult::success(format!(
        "Podcast created!\n\
             ID: {podcast_id}\n\
             Title: {}\n\
             Feed URL: {feed_url}\n\
             \n\
             Next: Add episodes with action 'create_episode' using this podcast_id.",
        params.title
    ))
    .with_metadata("podcast_id", json!(podcast_id))
    .with_metadata("feed_url", json!(feed_url)))
}
