//! Delay for a scheduled same-model reconnect attempt.
/// Return the original exponential backoff in seconds.
///
/// # Arguments
/// * `attempt` — One-based scheduled retry count.
///
/// # Returns
/// Ten seconds doubled for every additional attempt.
///
/// # Examples
/// ```
/// assert_eq!(codetether_stream_recovery::backoff_secs(3), 40);
/// ```
pub fn backoff_secs(attempt: u32) -> u64 {
    10 * 2u64.saturating_pow(attempt.saturating_sub(1))
}
