//! Interpret generate_from_episode service responses.
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn read(resp: reqwest::Response) -> Result<ToolResult> {
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Ok(ToolResult::error(format!(
            "Avatar episode generation failed ({status}): {body}"
        )));
    }

    let body: Value = resp.json().await.context("Failed to parse response")?;

    if let Some(error) = body["error"].as_str() {
        return Ok(ToolResult::error(format!(
            "Avatar generation error: {error}"
        )));
    }

    Ok(super::generate_from_episode_output::format(body))
}
