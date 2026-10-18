use super::{Host, MAX_FAILURE_TURNS, notify, record, status};
use crate::StepFlow;
/// Recover an active goal after exhausted retryable provider failures.
///
/// # Arguments
/// `host` owns native goal services; `error` is the exhausted provider error.
/// # Returns
/// Whether native continuation selected another goal turn. Lookup or
/// continuation failures return false; status persistence failures are logged.
/// # Examples
/// ```no_run
/// # async fn recover(host: &mut impl codetether_prompt_loop::goal_recovery::Host) {
/// let error = anyhow::anyhow!("service unavailable");
/// let _continued = codetether_prompt_loop::goal_recovery::after(host, &error).await;
/// # }
/// ```
pub async fn after(host: &mut impl Host, error: &anyhow::Error) -> bool {
    if !codetether_provider_policy::is_retryable_upstream_error(error) || !host.active().await {
        return false;
    }
    record::record(host.progress(), error);
    let attempt = host.progress().goal_failure_repeats;
    if attempt >= MAX_FAILURE_TURNS {
        status::persist(host).await;
        return false;
    }
    notify::send(host, error, attempt, MAX_FAILURE_TURNS - 1).await;
    tokio::time::sleep(std::time::Duration::from_secs(u64::from(attempt))).await;
    matches!(host.continue_goal().await, Ok(StepFlow::ContinueGoal))
}
