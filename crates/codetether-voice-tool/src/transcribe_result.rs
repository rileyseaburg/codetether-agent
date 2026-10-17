//! Decode transcription status, text and source metadata.
use super::params::TranscribeParams;
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(resp: reqwest::Response, params: &TranscribeParams) -> Result<ToolResult> {
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!(
            "Transcription API returned {status}: {body}"
        )));
    }

    let body: Value = resp.json().await.context("Failed to parse response")?;
    let text = body["transcription"]
        .as_str()
        .unwrap_or("(no transcription returned)");

    Ok(ToolResult::success(text).with_metadata("file", json!(params.file_path)))
}
