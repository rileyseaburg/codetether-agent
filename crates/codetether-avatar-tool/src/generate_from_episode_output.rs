//! Format a generated episode video and optional YouTube outcome.
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) fn format(body: Value) -> ToolResult {
    let video_path = body["video_path"].as_str().unwrap_or("unknown");
    let title = body["episode_title"].as_str().unwrap_or("unknown");
    let mut output = format!(
        "Avatar video generated from episode!\n\
             Title: {title}\n\
             Video: {video_path}"
    );

    if let Some(yt) = body.get("youtube") {
        let yt_url = yt["url"].as_str().unwrap_or("unknown");
        let yt_id = yt["video_id"].as_str().unwrap_or("unknown");
        output.push_str(&format!(
            "\n\nUploaded to YouTube!\nURL: {yt_url}\nVideo ID: {yt_id}"
        ));
    }

    if let Some(yt_err) = body["youtube_error"].as_str() {
        output.push_str(&format!("\n\nYouTube upload error: {yt_err}"));
    }

    ToolResult::success(output)
        .with_metadata("video_path", json!(video_path))
        .with_metadata("episode_title", json!(title))
}
