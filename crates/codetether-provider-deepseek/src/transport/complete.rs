//! Execute a non-streaming completion request.
use super::map_error::map_error;
use crate::{
    DeepSeekProvider, request,
    response::{self, DsResponse},
};
use anyhow::{Context, Result};
use codetether_provider_core::{CompletionRequest, CompletionResponse};
impl DeepSeekProvider {
    pub(crate) async fn complete_non_streaming(
        &self,
        req: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let body = request::build(&req);
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
        response::parse(ds)
    }
}
