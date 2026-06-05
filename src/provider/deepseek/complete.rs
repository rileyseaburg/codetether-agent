//! Non-streaming completion for DeepSeek.

use anyhow::{Context, Result};

use crate::provider::{CompletionRequest, CompletionResponse};

use super::{DeepSeekProvider, body, error::DsError, parse_response, response::DsResponse};

impl DeepSeekProvider {
    pub(crate) async fn complete_non_streaming(
        &self,
        req: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let body = body::build(&req);
        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
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
