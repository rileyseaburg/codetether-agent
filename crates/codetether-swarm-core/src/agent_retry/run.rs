use super::{AgentAttempt, AgentLoopExit, AgentOutcome, backoff, logging};
/// Retry a host attempt until it completes or reaches the retry limit.
///
/// # Arguments
/// * `agent` - Attempt implementation invoked for every try.
/// * `id` - Task identifier included in diagnostics.
/// * `max_retries` - Additional attempts allowed after the initial attempt.
/// * `base_delay_ms` - Initial backoff delay.
/// * `max_delay_ms` - Maximum backoff delay.
///
/// # Returns
/// The final outcome and number of retries performed.
///
/// # Examples
/// ```no_run
/// # async fn example(agent: &impl codetether_swarm_core::agent_retry::AgentAttempt) {
/// let (_, retries) = codetether_swarm_core::agent_retry::run(agent, "task", 0, 100, 1000).await;
/// assert_eq!(retries, 0);
/// # }
/// ```
pub async fn run(
    agent: &impl AgentAttempt,
    id: &str,
    max_retries: u32,
    base_delay_ms: u64,
    max_delay_ms: u64,
) -> (AgentOutcome, u32) {
    let mut attempt = 0;
    loop {
        let outcome = agent.execute().await;
        let complete = matches!(&outcome, Ok((_, _, _, AgentLoopExit::Completed)));
        if complete || attempt >= max_retries {
            if !complete && attempt >= max_retries {
                logging::exhausted(id, attempt, max_retries);
            }
            return (outcome, attempt);
        }
        let delay = backoff::calculate(attempt, base_delay_ms, max_delay_ms, 2.0);
        match &outcome {
            Ok((_, _, _, exit)) => {
                logging::incomplete(id, attempt, max_retries, *exit, delay.as_millis())
            }
            Err(error) => logging::error(id, attempt, max_retries, error, delay.as_millis()),
        }
        tokio::time::sleep(delay).await;
        attempt += 1;
    }
}
