//! Shared typed request preparation.
use crate::*;
impl StepFunProvider {
    pub(crate) fn build_request(&self, request: &CompletionRequest, stream: bool) -> ChatRequest {
        let messages = self.convert_messages(&request.messages);
        let tools = self.convert_tools(&request.tools);

        let chat_request = ChatRequest {
            model: request.model.clone(),
            messages,
            tools: if tools.is_empty() { None } else { Some(tools) },
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: Some(stream),
        };

        chat_request
    }
}
