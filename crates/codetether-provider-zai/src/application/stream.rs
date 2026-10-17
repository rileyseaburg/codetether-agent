//! Prepare and delegate an application stream request.
use super::ZaiProvider;
use crate::thinking::thinking_config_for_request;
use crate::{CompletionRequest, StreamChunk, stream_request};
use anyhow::Result;
impl ZaiProvider {
    pub(super) async fn stream_request(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        // Compatibility-first mode: omit historical reasoning_content from
        // request messages to avoid strict parameter validation errors on some
        // endpoint variants.
        let messages = Self::convert_messages(&request.messages, false);
        let tools = Self::convert_tools(&request.tools);
        let body = stream_request::build_body(
            &request,
            messages,
            tools,
            thinking_config_for_request(&request),
            Self::model_supports_tool_stream(&request.model),
        );

        tracing::debug!(model = %request.model, "Z.AI streaming request");

        crate::streaming::complete_application(&self.client, &self.api_key, &self.base_url, &body)
            .await
    }
}
