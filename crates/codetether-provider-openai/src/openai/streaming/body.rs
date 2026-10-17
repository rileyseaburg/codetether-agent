//! body for the OpenAI-compatible transport.

use super::*;

impl OpenAIProvider {
    pub(in crate::openai) fn stream_body(
        &self,
        request: &CompletionRequest,
    ) -> Result<async_openai::types::chat::CreateChatCompletionRequest> {
        let messages = Self::convert_messages(&request.messages)?;
        let tools = Self::convert_tools(&request.tools)?;

        let mut req_builder = CreateChatCompletionRequestArgs::default();
        let model_id = alias::normalize_model_id(&self.provider_name, &request.model);
        req_builder
            .model(model_id.as_ref())
            .messages(messages)
            .stream(true);

        if !tools.is_empty() {
            req_builder.tools(tools);
        }
        if let Some(temp) = request.temperature {
            req_builder.temperature(temp);
        }
        if let Some(max) = request.max_tokens {
            if self.provider_name == "openai" {
                req_builder.max_completion_tokens(max as u32);
            } else {
                req_builder.max_tokens(max as u32);
            }
        }

        Ok(req_builder.build()?)
    }
}
