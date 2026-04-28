//! play action — replay a previously generated TTS job in the browser.

use super::super::{Params, voice_api_url};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::json;

pub(crate) async fn run(client: &reqwest::Client, params: &Params) -> Result<ToolResult> {
    let job_id = match &params.job_id {
        Some(id) if !id.trim().is_empty() => id.clone(),
        _ => {
            return Ok(ToolResult::structured_error(
                "MISSING_PARAM",
                "voice_stream",
                "'job_id' is required for 'play'",
                Some(vec!["job_id"]),
                Some(json!({"action": "play", "job_id": "abc123"})),
            ));
        }
    };

    let base = voice_api_url();
    let output_url = format!("{base}/outputs/{job_id}");

    let check = client
        .head(&output_url)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to check output: {e}"))?;

    if !check.status().is_success() {
        return Ok(ToolResult::error(format!(
            "Output not found for job_id '{job_id}' (HTTP {})",
            check.status()
        )));
    }

    let launch = super::audio_player::open(&job_id, &output_url)?;
    let status = if launch.browser_opened {
        format!("Playing job {job_id} in browser.")
    } else {
        format!("Player saved locally for job {job_id} (browser launch unavailable).")
    };

    Ok(ToolResult::success(format!(
        "{status}\nURL: {output_url}\nPlayer: {}",
        launch.html_path.display()
    ))
    .with_metadata("job_id", json!(job_id))
    .with_metadata("output_url", json!(output_url))
    .with_metadata("player_path", json!(launch.html_path)))
}
