//! Native step, response and finalization effects for the shared lifecycle.
use super::super::super::Runner;
use crate::{provider::CompletionResponse, session::SessionResult};
use codetether_prompt_loop::{StepFlow, lifecycle::Effects};
impl Effects for Runner<'_> {
    async fn begin(&mut self, step: usize) -> anyhow::Result<()> {
        super::super::begin::run(self, step).await
    }
    async fn normalize(&mut self, response: CompletionResponse) -> CompletionResponse {
        super::super::super::response::normalize(self, response).await
    }
    async fn handle(
        &mut self,
        step: usize,
        response: CompletionResponse,
    ) -> anyhow::Result<StepFlow> {
        super::super::super::response::handle(self, step, response).await
    }
    fn clear_budget(&mut self) {
        crate::session::step_limit::clear_budget();
    }
    async fn finish(&mut self) -> anyhow::Result<SessionResult> {
        super::super::super::finish::finish(self).await
    }
}
