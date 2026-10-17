//! body for the OpenAI-compatible transport.

use super::*;

impl OpenAIProvider {
    pub(in crate::openai) fn completion_body(
        &self,
        request: &CompletionRequest,
        messages: Vec<ChatCompletionRequestMessage>,
        tools: Vec<ChatCompletionTools>,
    ) -> Result<async_openai::types::chat::CreateChatCompletionRequest> {
        let mut req_builder = CreateChatCompletionRequestArgs::default();
        let model_id = alias::normalize_model_id(&self.provider_name, &request.model);
        req_builder.model(model_id.as_ref()).messages(messages);

        // Pass tools to the API if provided
        if !tools.is_empty() {
            req_builder.tools(tools);
        }
        if let Some(temp) = request.temperature {
            req_builder.temperature(temp);
        }
        if let Some(top_p) = request.top_p {
            req_builder.top_p(top_p);
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
