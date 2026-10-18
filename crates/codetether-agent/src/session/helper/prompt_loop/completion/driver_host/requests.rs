//! Native context, request and provider bindings.
use super::super::super::Runner;
use crate::provider::{CompletionRequest, CompletionResponse};
use codetether_prompt_loop::completion::{Attempt, driver::Requests};
impl Requests for Runner<'_> {
    async fn initialize(&mut self, step: usize) -> anyhow::Result<Attempt> {
        super::super::context::new(self, step).await
    }
    async fn build(&mut self, attempt: &mut Attempt) -> anyhow::Result<CompletionRequest> {
        super::super::request::build(self, attempt).await
    }
    async fn dispatch(&mut self, request: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        super::super::super::super::prompt_call::complete_step(
            &self.inner.model.provider,
            request,
            &self.inner.session.id,
            self.inner.model.supports_tools,
            self.inner.events.as_ref(),
        )
        .await
    }
}
