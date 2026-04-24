//! HTTP POST to the Voice API `/transcribe` endpoint.

use anyhow::{Context, Result};
use serde_json::Value;

use crate::tool::ToolResult;

use super::super::client;

/// Send WAV audio to the transcription API and return the text.
pub(crate) async fn post_transcribe(
    client: &reqwest::Client,
    wav_bytes: &[u8],
) -> Result<ToolResult> {
    let base_url = client::api_url();
    let url = format!("{base_url}/transcribe");

    let part = reqwest::multipart::Part::bytes(wav_bytes.to_vec())
        .file_name("recording.wav")
        .mime_str("audio/wav")?;

    let form = reqwest::multipart::Form::new().part("audio_file", part);

    let resp = client
        .post(&url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Transcription request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!(
            "Voice API returned {status}: {body}. \
             Check CODETETHER_VOICE_API_URL."
        )));
    }

    let body: Value = resp.json().await.context("Failed to parse response")?;
    let text = body["transcription"]
        .as_str()
        .unwrap_or("(no transcription returned)");

    Ok(ToolResult::success(text.to_string()))
}
