//! Native adapter for shared completion recovery.
mod host;
use super::{super::Runner, context::Attempt};
pub(super) async fn recover(
    runner: &mut Runner<'_>,
    step: usize,
    attempt: &mut Attempt,
    error: &anyhow::Error,
) -> anyhow::Result<bool> {
    codetether_prompt_loop::completion::recover(&mut host::Native { runner, attempt }, step, error)
        .await
}
