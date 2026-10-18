//! Local job execution through an application-bound sub-agent loop.
//!
//! [execute] retains the shared job lifecycle, retry and workspace behavior.
//! [JobHost] selects native workspace factories and system instructions.
mod application;
mod attempt;
mod host;
mod registry;
use codetether_swarm_core::SubTaskResult;
use codetether_swarm_local_job::Job;
pub use host::JobHost;

/// Execute a prepared local job using the shared sub-agent loop.
/// # Arguments
/// `job` owns the provider, workspace, prompts, limits and lifecycle channels.
/// # Returns
/// The original job identifier and execution result.
/// # Errors
/// Errors are carried in the result tuple and retain the shared job policy.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_swarm_agent::job::JobHost>(job: codetether_swarm_local_job::Job) {
/// let (_id, _result) = codetether_swarm_agent::job::execute::<H>(job).await;
/// # }
/// ```
pub async fn execute<H: JobHost>(job: Job) -> (String, anyhow::Result<SubTaskResult>) {
    codetether_swarm_local_job::execute::<application::ApplicationJob<H>>(job).await
}
