//! Assistant-text extraction from TetherScript completion results.

use anyhow::{Result, anyhow};

pub(super) fn text(response: &serde_json::Value) -> Result<String> {
    let response = response.get("ok").unwrap_or(response);
    if let Some(text) = response.get("content").and_then(|value| value.as_str()) {
        return Ok(text.to_string());
    }
    if let Some(text) = response
        .get("choices")
        .and_then(|choices| choices.as_array())
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
    {
        return Ok(text.to_string());
    }
    Err(anyhow!(
        "tetherscript provider response did not include assistant text content: {response}"
    ))
}
