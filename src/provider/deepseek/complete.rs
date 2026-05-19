//! Non-streaming completion for DeepSeek.

use anyhow::{Context, Result};

use super::DeepSeekProvider;
use super::body;
use super::error::DsError;
use super::parse_response;
use super::response::DsResponse;

pub(crate) async fn exec(
    p: &DeepSeekProvider,
    req: crate::provider::CompletionRequest,
) -> Result<crate::provider::CompletionResponse> {
    let body = body::build(&req);
    let resp = p
        .client
        .post(format!("{}/chat/completions", p.base_url))
        .header("Authorization", format!("Bearer {}", p.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .context("DeepSeek request failed")?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .context("Failed to read DeepSeek response")?;
    if !status.is_success() {
        return Err(map_error(status.as_u16(), &text));
    }
    let ds: DsResponse = serde_json::from_str(&text).context(format!(
        "Failed to parse DeepSeek response: {}",
        &text[..text.len().min(200)]
    ))?;
    parse_response::parse(ds)
}

fn map_error(code: u16, body: &str) -> anyhow::Error {
    if let Ok(err) = serde_json::from_str::<DsError>(body) {
        anyhow::anyhow!(
            "DeepSeek API error: {} ({:?})",
            err.error.message,
            err.error.error_type
        )
    } else {
        anyhow::anyhow!("DeepSeek API error: {code} {body}")
    }
}
