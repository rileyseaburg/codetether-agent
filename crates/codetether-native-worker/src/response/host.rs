//! Native control, telemetry, transcript and delegated tool-call effects.
use super::{ContextHost, Native, Tools, transcript};
use async_trait::async_trait;
use codetether_a2a_worker_core::session_protocol::{ToolCall, response::ResponseHost};
use codetether_native_providers::Message;
#[async_trait]
impl<S, R, E> ResponseHost for Native<'_, '_, S, R, E>
where
    S: ContextHost + transcript::Host,
    R: Sync,
    E: Tools<S, R>,
{
    async fn ensure_running(&mut self) -> anyhow::Result<()> {
        codetether_native_session_runtime::boundary::ensure_running(self.session.session_id()).await
    }
    fn record_usage(&mut self, model: &str, prompt_tokens: u64, completion_tokens: u64) {
        codetether_native_telemetry::TOKEN_USAGE.record_model_usage(
            model,
            prompt_tokens,
            completion_tokens,
        );
    }
    fn append_message(&mut self, message: Message) {
        transcript::Host::append(self.session, message);
    }
    async fn execute_tool_call(&mut self, tool_call: ToolCall) {
        E::execute(self.session, self.context, tool_call).await;
    }
}
