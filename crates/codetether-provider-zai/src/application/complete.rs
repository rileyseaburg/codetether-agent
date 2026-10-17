//! Prepare and delegate an application complete request.
use super::ZaiProvider;
use crate::thinking::thinking_config_for_request;
use crate::{CompletionRequest, CompletionResponse};
use anyhow::Result;
impl ZaiProvider {
    pub(super) async fn complete_request(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse> {
        // Compatibility-first mode: omit historical reasoning_content from
        // request messages to avoid strict parameter validation errors on some
        // endpoint variants.
        let messages = Self::convert_messages(&request.messages, false);
        let tools = Self::convert_tools(&request.tools);

        let body = crate::request::encode_buffered(
            &request,
            messages,
            tools,
            thinking_config_for_request(&request),
            crate::request::BufferedFormat::Application,
        );

        tracing::debug!(model = %request.model, "Z.AI request");
        tracing::trace!(body = %serde_json::to_string(&body).unwrap_or_default(), "Z.AI request body");

        crate::completion::complete_application(&self.client, &self.api_key, &self.base_url, &body)
            .await
    }
}
