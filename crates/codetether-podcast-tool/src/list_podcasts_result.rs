//! Decode and format a podcast listing.
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(resp: reqwest::Response) -> Result<ToolResult> {
    let body: Value = resp.json().await.context("Failed to parse response")?;
    let podcasts = body["podcasts"].as_array().or_else(|| body.as_array());

    match podcasts {
        Some(podcasts) if !podcasts.is_empty() => {
            let mut output = format!("Found {} podcast(s):\n\n", podcasts.len());
            for p in podcasts {
                let id = p["podcast_id"].as_str().unwrap_or("?");
                let title = p["title"].as_str().unwrap_or("Untitled");
                let eps = p["episode_count"].as_u64().unwrap_or(0);
                let feed = p["feed_url"].as_str().unwrap_or("?");
                output.push_str(&format!(
                    "- **{title}** (id: {id}, {eps} episodes)\n  Feed: {feed}\n"
                ));
            }
            Ok(ToolResult::success(output).with_metadata("count", json!(podcasts.len())))
        }
        _ => Ok(ToolResult::success(
            "No podcasts found. Create one with action 'create_podcast'.",
        )),
    }
}
