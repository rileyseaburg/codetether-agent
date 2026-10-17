//! Apply the optional host tool-call reformatter.
use super::*;
impl RlmExecutor {
    pub(super) async fn rewrite_analysis(
        &self,
        response: codetether_provider_core::CompletionResponse,
        tools: &[codetether_provider_core::ToolDefinition],
    ) -> codetether_provider_core::CompletionResponse {
        let response = if let Some(ref router) = self.tool_router {
            // RLM executor currently uses the same provider as the main session,
            // which always supports native tool calling.  Pass `true` so
            // FunctionGemma is skipped — it would only waste CPU here.
            router.maybe_reformat(response, tools, true).await
        } else {
            response
        };

        response
    }
}
