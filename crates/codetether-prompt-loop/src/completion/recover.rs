//! Ordered context, server-delay and upstream-error recovery.
use super::Host;
/// Recover a failed provider completion attempt.
/// # Arguments
/// `host` supplies attempt state and native effects; `step` identifies the
/// prompt step, and `error` is the provider or local request-guard failure.
/// # Returns
/// Whether another attempt should be made.
/// # Errors
/// Propagates derivation and failover setup errors.
/// # Examples
/// ```no_run
/// # async fn retry(host: &mut impl codetether_prompt_loop::completion::Host) -> anyhow::Result<()> {
/// let error = anyhow::anyhow!("service unavailable");
/// let _retry = codetether_prompt_loop::completion::recover(host, 1, &error).await?;
/// # Ok(()) }
/// ```
pub async fn recover(
    host: &mut impl Host,
    step: usize,
    error: &anyhow::Error,
) -> anyhow::Result<bool> {
    if let Some((policy, keep)) =
        super::derivation(error, host.attempt().count, host.attempt().policy)
    {
        let derived = host.derive(policy, keep).await?;
        host.attempt().derived = derived;
        return Ok(true);
    }
    if let Some(seconds) = super::retry_after::seconds(error) {
        tracing::debug!(backoff_secs = seconds, "Provider requested a delayed retry");
        super::interlude::wait(host, seconds).await;
        return Ok(true);
    }
    if host.attempt().upstream_retries >= 3
        || !codetether_provider_policy::is_retryable_upstream_error(error)
    {
        return Ok(false);
    }
    host.failed();
    host.attempt().upstream_retries += 1;
    let seconds = 1u64 << (host.attempt().upstream_retries - 1).min(2);
    let retry = host.attempt().upstream_retries;
    tracing::warn!(error = %error, retry = retry,
        backoff_secs = seconds, "Retryable upstream provider error");
    tokio::time::sleep(std::time::Duration::from_secs(seconds)).await;
    if host.failover()? {
        let policy = host.attempt().policy;
        let derived = host.derive(policy, None).await?;
        host.attempt().derived = derived;
        let proactive = host.proactive(step).await;
        host.attempt().proactive = proactive;
        host.attempt().count = 0;
    }
    Ok(true)
}
