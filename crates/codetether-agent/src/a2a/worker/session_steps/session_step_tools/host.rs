//! Application session-aware enrichment for native worker tool calls.
use crate::session::Session;
#[async_trait::async_trait]
impl codetether_native_worker::tool_call::InputContext for Session {
    fn record_outcome(&mut self, id: &str, result: &codetether_tool_core::ToolResult) {
        let source_turn = self.message_count();
        self.metadata
            .progress
            .observe_outcome(id, &result.output, result.success, source_turn);
    }
    fn repeated_edit(&self, tool: &str, input: &serde_json::Value) -> Option<String> {
        self.metadata.progress.blocked_edit(tool, input)
    }
    async fn refresh_context(&mut self) -> anyhow::Result<()> {
        self.refresh_context_from_store().await
    }
    fn prepare_worker_input(
        &self,
        input: &serde_json::Value,
        workspace: &std::path::Path,
        model: &str,
    ) -> serde_json::Value {
        super::super::super::enrich_tool_input_for_session_model(
            input,
            workspace,
            self,
            Some(model),
        )
    }
}
