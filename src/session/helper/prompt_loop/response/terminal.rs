//! Native entry point for shared terminal-response sequencing.
use super::super::{Runner, state::StepFlow};
use crate::provider::CompletionResponse;
#[path = "terminal/host.rs"]
mod host;
pub(super) async fn finish_or_retry(
    runner: &mut Runner<'_>,
    response: CompletionResponse,
) -> anyhow::Result<StepFlow> {
    codetether_prompt_loop::terminal::finish_or_retry(runner, response).await
}
