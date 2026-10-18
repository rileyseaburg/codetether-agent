//! Bind shared job execution to native workspace tools and the agent loop.
use super::{job::Job, job_host::Native};
pub(in crate::swarm::executor) async fn execute(
    job: Job,
) -> (String, anyhow::Result<crate::swarm::SubTaskResult>) {
    codetether_swarm_local_job::execute::<Native>(job).await
}
