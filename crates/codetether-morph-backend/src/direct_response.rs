//! Decode the direct endpoint response and reject failed or empty completions.
use super::response::MorphResponse;
use anyhow::{Context, Result, anyhow};
pub(super) fn parse(status: reqwest::StatusCode, text: String) -> Result<String> {
    if !status.is_success() {
        anyhow::bail!("Morph backend error: {} {}", status, text);
    }

    let parsed: MorphResponse =
        serde_json::from_str(&text).context("Failed to parse Morph response JSON")?;
    let content = parsed
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| anyhow!("Morph backend returned empty content"))?;

    Ok(content)
}
