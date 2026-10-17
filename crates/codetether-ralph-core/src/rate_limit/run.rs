//! Retry an operation only for the original rate-limit text patterns.
use std::future::Future;
/// Execute an operation with the configured rate-limit retry budget.
///
/// # Arguments
/// * `max_rate_limit_retries` - Number of retries after the initial attempt.
/// * `rate_limit_base_delay_ms` - Base delay multiplied by the exponential factor.
/// * `story_id` - Story identifier included in backoff logs.
/// * `operation` - Async operation invoked for each attempt.
/// # Returns
/// The first successful output or the final operation error.
/// # Errors
/// Returns non-rate-limit errors immediately, or the last error when retries end.
/// # Panics
/// Delay multiplication may panic on overflow when overflow checks are enabled.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let output = codetether_ralph_core::rate_limit::retry(2, 100, "US-1", || async { Ok("done".to_string()) }).await?;
/// assert_eq!(output, "done");
/// # Ok(()) }
/// ```
pub async fn retry<F, Fut>(
    max_rate_limit_retries: usize,
    rate_limit_base_delay_ms: u64,
    story_id: &str,
    mut operation: F,
) -> anyhow::Result<String>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = anyhow::Result<String>>,
{
    let mut last_err = None;
    let mut attempt_result = None;
    for attempt in 0..=max_rate_limit_retries {
        let res = operation().await;
        match res {
            Ok(output) => {
                attempt_result = Some(Ok(output));
                break;
            }
            Err(e) => {
                let err_str = format!("{e}");
                let is_rate_limit = err_str.contains("Rate limit")
                    || err_str.contains("rate_limit")
                    || err_str.contains("429")
                    || err_str.contains("too many requests");
                if is_rate_limit && attempt < max_rate_limit_retries {
                    super::backoff::pause(
                        attempt,
                        max_rate_limit_retries,
                        rate_limit_base_delay_ms,
                        story_id,
                    )
                    .await;
                } else {
                    last_err = Some(e);
                    break;
                }
            }
        }
    }
    attempt_result.unwrap_or_else(|| {
        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("All retries exhausted")))
    })
}
