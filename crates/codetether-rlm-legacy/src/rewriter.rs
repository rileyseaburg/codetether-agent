//! Supply the application tool router to the shared executor.
use super::*;
/// Local adapter joining the shared router with the shared RLM interface.
pub(super) struct RouterRewriter(pub(super) ToolCallRouter);
#[async_trait::async_trait]
impl ResponseRewriter for RouterRewriter {
    async fn maybe_reformat(
        &self,
        response: codetether_provider_core::CompletionResponse,
        tools: &[codetether_provider_core::ToolDefinition],
        model_supports_tools: bool,
    ) -> codetether_provider_core::CompletionResponse {
        ToolCallRouter::maybe_reformat(&self.0, response, tools, model_supports_tools).await
    }
}
