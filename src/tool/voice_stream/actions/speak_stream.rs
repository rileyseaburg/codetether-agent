//! speak_stream action — POST /tts/speak, open browser player.

use super::super::{Params, voice_api_url};
use crate::tool::ToolResult;
use anyhow::{Context, Result};

pub(crate) async fn run(client: &reqwest::Client, params: &Params) -> Result<ToolResult> {
    let Some(text) = params.text.as_ref().filter(|t| !t.trim().is_empty()) else {
        return Ok(super::tts_result::missing_text());
    };
    let base = voice_api_url();
    let resp = super::tts_request::send(client, params, &base, text).await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!("TTS API {status}: {err}")));
    }
    let data: serde_json::Value = resp.json().await.context("Failed to parse TTS response")?;
    let job_id = data["job_id"].as_str().unwrap_or("unknown");
    let url = data["output_url"]
        .as_str()
        .map(str::to_string)
        .unwrap_or_else(|| format!("{base}/outputs/{job_id}"));
    super::tts_result::success(job_id, &url)
}
