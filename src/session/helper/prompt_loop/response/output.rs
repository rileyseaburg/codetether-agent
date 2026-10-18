//! Native field projection for shared assistant-output handling.
use super::super::Runner;
use crate::provider::CompletionResponse;
pub(super) async fn emit(runner: &mut Runner<'_>, step: usize, response: &CompletionResponse) {
    codetether_prompt_loop::output::emit(
        codetether_prompt_loop::output::Context {
            output: &mut runner.progress.output,
            events: runner.events.as_ref(),
            bus: runner.session.bus.as_ref(),
            agent: &runner.session.agent,
            turn_id: &runner.progress.turn_id,
            step,
        },
        response,
    )
    .await;
}
