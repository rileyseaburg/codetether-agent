//! Bind a spawned child job to its parent's shared runtime capability.

use super::job::Job;
use codetether_swarm_core::SubTaskResult;

/// Execute the job under its parent runtime capability when available.
/// # Arguments
/// `job` contains the owned task inputs and execution limits.
/// # Returns
/// The task ID with its result, or an error if concurrency acquisition fails.
/// # Examples
/// ```no_run
/// async fn run<H: codetether_swarm_local_job::Host>(job: codetether_swarm_local_job::Job) {
///     let (_id, _result) = codetether_swarm_local_job::execute::<H>(job).await;
/// }
/// ```
pub async fn execute<H: crate::Host>(job: Job) -> (String, anyhow::Result<SubTaskResult>) {
    match job.control.clone() {
        Some(control) => control.scope(super::job_execute::execute::<H>(job)).await,
        None => super::job_execute::execute::<H>(job).await,
    }
}
