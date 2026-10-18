//! Bound one completion future by the remaining sub-agent deadline.
use crate::agent_loop::RequestOutcome as Outcome;
use codetether_provider_core::CompletionResponse;
use std::{
    future::Future,
    time::{Duration, Instant},
};
use tokio::time::timeout;

/// Execute a completion under the agent's absolute deadline.
/// # Arguments
/// * `deadline` — End of the agent's allowed run time.
/// * `timeout_secs` — Configured limit reported in timeout diagnostics.
/// * `completion` — Provider completion and accounting future.
/// # Returns
/// A response outcome or a timeout outcome.
/// # Errors
/// Propagates completion failures when the deadline did not elapse first.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let result = codetether_swarm_core::agent_loop::request::execute(
///     std::time::Instant::now(), 0, std::future::pending(),
/// ).await?;
/// assert!(matches!(result, codetether_swarm_core::agent_loop::RequestOutcome::TimedOut));
/// # Ok(()) }
/// ```
pub async fn execute<F>(
    deadline: Instant,
    timeout_secs: u64,
    completion: F,
) -> anyhow::Result<Outcome>
where
    F: Future<Output = anyhow::Result<CompletionResponse>>,
{
    let remaining = remaining(deadline, Instant::now());
    match timeout(remaining, completion).await {
        Ok(response) => Ok(Outcome::Response(response?)),
        Err(_) => {
            tracing::warn!(
                target: "codetether_agent::swarm::executor::agent_loop::request",
                timeout_secs = timeout_secs,
                "Sub-agent provider request timed out"
            );
            Ok(Outcome::TimedOut)
        }
    }
}

fn remaining(deadline: Instant, now: Instant) -> Duration {
    deadline.saturating_duration_since(now)
}

#[cfg(test)]
#[path = "remaining_tests.rs"]
mod tests;
