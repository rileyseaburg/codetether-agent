//! Provider request attempts and delegation accounting.

mod context;
mod derived;
mod failover;
mod reasoning_context;
mod recovery;
mod request;

use super::Runner;
use crate::provider::CompletionResponse;
use crate::session::delegation_skills;
use anyhow::Result;

/// Obtains one successful completion, retrying recoverable failures.
///
/// # Errors
///
/// Returns an unrecoverable provider or context-derivation error.
pub(super) async fn complete(runner: &mut Runner<'_>, step: usize) -> Result<CompletionResponse> {
    let mut attempt = context::Attempt::new(runner, step).await?;
    let started = std::time::Instant::now();
    loop {
        attempt.count += 1;
        let request = request::build(runner, &mut attempt).await;
        let result = super::super::prompt_call::complete_step(
            &runner.model.provider,
            request,
            &runner.session.id,
            runner.model.supports_tools,
            runner.events.as_ref(),
        )
        .await;
        match result {
            Ok(response) => {
                runner.session.metadata.delegation.update(
                    &runner.model.provider_name,
                    delegation_skills::MODEL_CALL,
                    attempt.bucket,
                    true,
                );
                super::response::usage(runner, &response, started.elapsed()).await;
                runner.progress.goal_failure_signature = None;
                runner.progress.goal_failure_repeats = 0;
                return Ok(response);
            }
            Err(error) => {
                super::super::prompt_call::persist_stream_checkpoints(runner.session, &error)
                    .await?;
                if !recovery::recover(runner, step, &mut attempt, &error).await? {
                    return Err(error);
                }
            }
        }
    }
}
