//! Native adapter for ordered completion usage accounting.
mod host;
use super::super::Runner;
use crate::provider::CompletionResponse;
use std::time::Duration;
pub(super) async fn record(
    runner: &mut Runner<'_>,
    response: &CompletionResponse,
    elapsed: Duration,
) {
    codetether_prompt_loop::usage::record(runner, response, elapsed).await;
}
