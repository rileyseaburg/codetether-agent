//! Native worker completion and goal-accounting services.
use super::super::session_context::Context;
use crate::{provider::CompletionResponse, session::Session};
use std::{sync::Arc, time::Duration};
pub(super) struct Native<'a, 'context> {
    pub context: &'a Context<'context>,
    pub session: &'a mut Session,
}
impl codetether_a2a_worker_core::session_protocol::completion::accounted::Host for Native<'_, '_> {
    fn model(&self) -> &str {
        self.context.model
    }
    fn compose(&self) -> String {
        crate::session::runtime::goals::compose(self.context.system_prompt, &self.session.id)
    }
    async fn complete(
        &mut self,
        prompt: &str,
        temperature: Option<f32>,
    ) -> anyhow::Result<CompletionResponse> {
        super::super::super::complete_worker_step_with_context_fallback(
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
        crate::session::runtime::goals::record_usage(&self.session.id, tokens, elapsed).await
    }
}
