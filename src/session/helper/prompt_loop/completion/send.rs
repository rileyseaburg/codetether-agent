//! Guard request construction before dispatching a provider attempt.

use super::{super::Runner, context::Attempt};
use crate::provider::CompletionResponse;
use anyhow::Result;

/// Return local context-budget errors through the same recovery path as provider errors.
pub(super) async fn run(
    runner: &mut Runner<'_>,
    attempt: &mut Attempt,
) -> Result<CompletionResponse> {
    let request = super::request::build(runner, attempt).await?;
    super::super::super::prompt_call::complete_step(
        &runner.model.provider,
        request,
        &runner.session.id,
        runner.model.supports_tools,
        runner.events.as_ref(),
    )
    .await
}
