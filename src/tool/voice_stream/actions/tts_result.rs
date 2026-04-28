//! TTS success/error formatting for voice stream.

use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::json;

pub(super) fn missing_text() -> ToolResult {
    ToolResult::structured_error(
        "MISSING_PARAM",
        "voice_stream",
        "'text' is required for 'speak_stream'",
        Some(vec!["text"]),
        Some(json!({"action":"speak_stream","text":"Hello"})),
    )
}

pub(super) fn success(job_id: &str, output_url: &str) -> Result<ToolResult> {
    let launch = super::audio_player::open(job_id, output_url)?;
    let status = if launch.browser_opened {
        "Speech opened in browser."
    } else {
        "Speech player saved locally (browser launch unavailable)."
    };
    let msg = format!(
        "{status}\nJob ID: {job_id}\nURL: {output_url}\nPlayer: {}",
        launch.html_path.display()
    );
    Ok(ToolResult::success(msg)
        .with_metadata("job_id", json!(job_id))
        .with_metadata("output_url", json!(output_url))
        .with_metadata("player_path", json!(launch.html_path)))
}
