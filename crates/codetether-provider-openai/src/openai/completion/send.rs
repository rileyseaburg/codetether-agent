//! send for the OpenAI-compatible transport.

use super::*;

impl OpenAIProvider {
    pub(in crate::openai) async fn send_completion(
        &self,
        request: &CompletionRequest,
    ) -> Result<async_openai::types::chat::CreateChatCompletionResponse> {
        let messages = Self::convert_messages(&request.messages)?;
        let tools = Self::convert_tools(&request.tools)?;

        let primary_request = self.completion_body(request, messages.clone(), tools)?;
        let response = match self.client.chat().create(primary_request).await {
            Ok(response) => response,
            Err(err)
                if self.provider_name == "minimax"
                    && minimax_error::is_minimax_chat_setting_error(&err.to_string()) =>
            {
                tracing::warn!(
                    provider = "minimax",
                    error = %err,
                    "MiniMax rejected chat settings; retrying with conservative defaults"
                );

                let mut fallback_builder = CreateChatCompletionRequestArgs::default();
                fallback_builder.model(&request.model).messages(messages);
                self.client.chat().create(fallback_builder.build()?).await?
            }
            Err(err) => return Err(err.into()),
        };

        Ok(response)
    }
}
