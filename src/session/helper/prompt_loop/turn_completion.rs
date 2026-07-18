//! Completion dispatch with goal-aware whole-turn recovery.

use super::Runner;
use crate::provider::CompletionResponse;
use anyhow::Result;

pub(super) async fn next(
    runner: &mut Runner<'_>,
    step: usize,
) -> Result<Option<CompletionResponse>> {
    match super::completion::complete(runner, step).await {
        Ok(response) => Ok(Some(response)),
        Err(error) if super::goal_recovery::after(runner, &error).await => Ok(None),
        Err(error) => Err(error),
    }
}
