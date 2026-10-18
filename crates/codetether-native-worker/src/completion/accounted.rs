//! Native goal-aware worker completion and durable usage accounting.
//! Native worker completion and goal-accounting services.
use codetether_a2a_worker_core::session_protocol::context::Context;
use codetether_native_context::ContextHost;
use codetether_native_providers::CompletionResponse;
use std::{sync::Arc, time::Duration};
struct Native<'a, 'context, S, R> {
    pub context: &'a Context<'context, R>,
    pub session: &'a mut S,
}
impl<S: ContextHost, R: Sync>
    codetether_a2a_worker_core::session_protocol::completion::accounted::Host
    for Native<'_, '_, S, R>
{
    fn model(&self) -> &str {
        self.context.model
    }
    fn compose(&self) -> String {
        codetether_native_goals::runtime::compose(
            self.context.system_prompt,
            self.session.session_id(),
        )
    }
    async fn complete(
        &mut self,
        prompt: &str,
        temperature: Option<f32>,
    ) -> anyhow::Result<CompletionResponse> {
        super::complete_worker_step_with_context_fallback(
            Arc::clone(&self.context.provider),
            self.session,
            self.context.model,
            prompt,
            self.context.tool_definitions,
            temperature,
        )
        .await
    }
    async fn account(&mut self, tokens: usize, elapsed: Duration) -> anyhow::Result<()> {
        codetether_native_goals::runtime::record_usage(self.session.session_id(), tokens, elapsed)
            .await
    }
}

/// Complete a worker step and account its usage against the current goal.
/// # Errors
/// Propagates completion or durable goal-accounting errors.
pub async fn complete<S: ContextHost, R: Sync>(
    context: &Context<'_, R>,
    session: &mut S,
) -> anyhow::Result<CompletionResponse> {
    codetether_a2a_worker_core::session_protocol::completion::accounted::complete(&mut Native {
        context,
        session,
    })
    .await
}
