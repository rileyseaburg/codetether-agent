//! Non-streaming completion execution for Anthropic-compatible providers.

use anyhow::Result;

use crate::provider::{CompletionRequest, CompletionResponse};

use super::{AnthropicProvider, body, complete_http, parse_response, response::AnthropicResponse};

impl AnthropicProvider {
    /// Execute a Messages API request and parse the provider response.
    pub(crate) async fn complete_non_streaming(
        &self,
        req: CompletionRequest,
    ) -> Result<CompletionResponse> {
        log_start(self, &req);
        self.validate_api_key()?;
        let body = body::build(&req, self.enable_prompt_caching);
        let response = complete_http::send(self, &body).await?;
        log_response(&response, &req);
        Ok(parse_response::parse(response))
    }
}

fn log_start(p: &AnthropicProvider, req: &CompletionRequest) {
    tracing::debug!(provider = %p.provider_name, model = %req.model,
        message_count = req.messages.len(), tool_count = req.tools.len(),
        "Starting completion request");
    tracing::debug!(model = %req.model, "Anthropic request");
}

fn log_response(response: &AnthropicResponse, req: &CompletionRequest) {
    tracing::debug!(response_id = %response.id, model = %response.model,
        stop_reason = ?response.stop_reason, request_model = %req.model,
        "Received Anthropic response");
}

/// Return a character-safe prefix of a string.
pub(crate) fn safe_char_prefix(input: &str, max_chars: usize) -> String {
    input.chars().take(max_chars).collect()
}
