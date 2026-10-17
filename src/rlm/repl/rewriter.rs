//! Supply the application tool router to the shared executor.
use super::*;
#[async_trait::async_trait]
impl ResponseRewriter for ToolCallRouter {
    async fn maybe_reformat(
        &self,
        response: crate::provider::CompletionResponse,
        tools: &[crate::provider::ToolDefinition],
        model_supports_tools: bool,
    ) -> crate::provider::CompletionResponse {
        ToolCallRouter::maybe_reformat(self, response, tools, model_supports_tools).await
    }
}
