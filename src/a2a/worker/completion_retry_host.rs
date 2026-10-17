//! Application provider ownership for initial and final completion attempts.
use super::*;
use async_trait::async_trait;
use codetether_a2a_worker_core::session_protocol::completion::CompletionHost;
#[async_trait]
impl CompletionHost for ApplicationHost<'_> {
    async fn complete_initial(&mut self, tools: &[ToolDefinition]) -> Result<CompletionResponse> {
        crate::session::context::complete_with_context(
            Arc::clone(&self.provider),
            self.session,
            self.model,
            self.system_prompt,
            tools,
            self.options,
        )
        .await
    }
    async fn complete_retry(self, tools: &[ToolDefinition]) -> Result<CompletionResponse> {
        crate::session::context::complete_with_context(
            self.provider,
            self.session,
            self.model,
            self.system_prompt,
            tools,
            self.options,
        )
        .await
    }
}
