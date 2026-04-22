//! speak_stream action — POST /tts/speak, open browser player.

use super::super::{Params, voice_api_url};
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use serde_json::json;

pub(crate) async fn run(client: &reqwest::Client, params: &Params) -> Result<ToolResult> {
    let text = match &params.text {
        Some(t) if !t.trim().is_empty() => t.clone(),
        _ => {
            return Ok(ToolResult::structured_error(
                "MISSING_PARAM",
                "voice_stream",
                "'text' is required for 'speak_stream'",
                Some(vec!["text"]),
                Some(json!({"action": "speak_stream", "text": "Hello"})),
            ));
        }
    };

    let base = voice_api_url();
    let vid = params.voice_id.as_deref().unwrap_or("960f89fc");
    let lang = params.language.as_deref().unwrap_or("english");

    let body = json!({"script": text, "voice_id": vid, "language": lang});

    let resp = client
        .post(format!("{base}/tts/speak"))
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("TTS request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!("TTS API {status}: {err}")));
    }

    let data: serde_json::Value = resp.json().await.context("Failed to parse TTS response")?;
    let job_id = data["job_id"].as_str().unwrap_or("unknown");
    let output_url = data["output_url"]
        .as_str()
        .unwrap_or(&format!("{base}/outputs/{job_id}"));

    super::audio_player::open(job_id, output_url)?;

    Ok(ToolResult::success(format!(
        "Speech opened in browser.\nJob ID: {job_id}\nURL: {output_url}"
    ))
    .with_metadata("job_id", json!(job_id))
    .with_metadata("output_url", json!(output_url)))
}
