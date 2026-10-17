//! Decode create_episode responses and result metadata.
use super::params::CreateEpisodeParams;
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(
    resp: reqwest::Response,
    params: &CreateEpisodeParams,
) -> Result<ToolResult> {
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!(
            "Create episode failed ({status}): {body}"
        )));
    }

    let body: Value = resp.json().await.context("Failed to parse response")?;
    let episode_id = body["episode_id"].as_str().unwrap_or("unknown");
    let audio_url = body["audio_url"].as_str().unwrap_or("unknown");
    let duration = body["duration"].as_str().unwrap_or("unknown");
    let feed_url = body["feed_url"].as_str().unwrap_or("unknown");

    Ok(ToolResult::success(format!(
        "Episode created!\n\
             Episode ID: {episode_id}\n\
             Title: {}\n\
             Duration: {duration}\n\
             Audio: {audio_url}\n\
             Feed: {feed_url}\n\
             \n\
             The RSS feed has been updated automatically.",
        params.title
    ))
    .with_metadata("episode_id", json!(episode_id))
    .with_metadata("audio_url", json!(audio_url))
    .with_metadata("duration", json!(duration)))
}
