//! Application effects for shared worker response sequencing.
use super::ApplicationHost;
use crate::provider::Message;
use async_trait::async_trait;
use codetether_a2a_worker_core::session_protocol::{ToolCall, response::ResponseHost};
#[async_trait]
impl ResponseHost for ApplicationHost<'_, '_> {
    async fn ensure_running(&mut self) -> anyhow::Result<()> {
        crate::session::runtime::boundary::ensure_running(&self.session.id).await
    }
    fn record_usage(&mut self, model: &str, prompt_tokens: u64, completion_tokens: u64) {
        crate::telemetry::TOKEN_USAGE.record_model_usage(model, prompt_tokens, completion_tokens);
    }
    fn append_message(&mut self, message: Message) {
        self.session.add_message(message);
    }
    async fn execute_tool_call(&mut self, tool_call: ToolCall) {
        super::super::session_step_tools::execute_tool_call(
            self.session,
            self.context.tool_registry,
            self.context.auto_approve,
            self.context.workspace_dir,
            self.context.model,
            &self.context.output_callback,
            tool_call,
        )
        .await;
    }
}
