//! Native session-control and response-effect bindings.
use super::super::super::Runner;
use crate::provider::CompletionResponse;
use codetether_prompt_loop::{
    StepFlow,
    driver::{Effects, ParsedCall},
};
impl Effects for Runner<'_> {
    async fn ensure_running(&mut self) -> anyhow::Result<()> {
        crate::session::runtime::boundary::ensure_running(&self.session.id).await
    }
    async fn emit(&mut self, step: usize, response: &CompletionResponse) {
        super::super::output::emit(self, step, response).await;
    }
    async fn terminal(&mut self, response: CompletionResponse) -> anyhow::Result<StepFlow> {
        super::super::terminal::finish_or_retry(self, response).await
    }
    async fn repair(
        &mut self,
        response: &CompletionResponse,
        calls: &[ParsedCall],
        truncated: &[(String, String)],
    ) -> bool {
        super::super::truncation::record(self, response, calls, truncated).await
    }
}
