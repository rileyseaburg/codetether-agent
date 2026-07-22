//! Recovery from context-window and transient upstream failures.

use super::{super::Runner, context::Attempt};
use crate::session::delegation_skills;
use anyhow::Result;

/// Attempts context compaction, backoff, and provider failover.
///
/// # Errors
///
/// Returns an error when re-derivation or failover setup fails.
pub(super) async fn recover(
    runner: &mut Runner<'_>,
    step: usize,
    attempt: &mut Attempt,
    error: &anyhow::Error,
) -> Result<bool> {
    if let Some((policy, keep)) =
        super::super::super::prompt_too_long::derivation(error, attempt.count, attempt.policy)
    {
        attempt.derived = super::derived::derive(runner, policy, keep).await?;
        return Ok(true);
    }
    if let Some(seconds) = super::retry_after::seconds(error) {
        tracing::debug!(backoff_secs = seconds, "Provider requested a delayed retry");
        super::interlude::wait(runner, seconds).await;
        return Ok(true);
    }
    if attempt.upstream_retries >= 3
        || !super::super::super::error::is_retryable_upstream_error(error)
    {
        return Ok(false);
    }
    runner.session.metadata.delegation.update(
        &runner.model.provider_name,
        delegation_skills::MODEL_CALL,
        attempt.bucket,
        false,
    );
    attempt.upstream_retries += 1;
    let seconds = 1u64 << (attempt.upstream_retries - 1).min(2);
    tracing::warn!(error = %error, retry = attempt.upstream_retries,
        backoff_secs = seconds, "Retryable upstream provider error");
    tokio::time::sleep(std::time::Duration::from_secs(seconds)).await;
    if super::failover::apply(runner, attempt.bucket)? {
        attempt.derived = super::derived::derive(runner, attempt.policy, None).await?;
        attempt.proactive = super::derived::lsp(runner, step).await;
        attempt.count = 0;
    }
    Ok(true)
}
