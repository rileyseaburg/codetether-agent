//! Native field projection for shared assistant-output handling.
use super::super::Runner;
use crate::provider::CompletionResponse;
pub(super) async fn emit(runner: &mut Runner<'_>, step: usize, response: &CompletionResponse) {
    codetether_prompt_loop::output::emit(
        codetether_prompt_loop::output::Context {
            output: &mut runner.inner.progress.output,
            events: runner.inner.events.as_ref(),
            bus: runner.inner.session.bus.as_ref(),
            agent: &runner.inner.session.agent,
            turn_id: &runner.inner.progress.turn_id,
            step,
        },
        response,
    )
    .await;
}
