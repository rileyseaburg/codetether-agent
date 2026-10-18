//! Native agent execution for local stages.
use super::Native;
impl codetether_swarm_local_stage::Execution for Native<'_> {
    async fn run_job(
        job: codetether_swarm_local_job::Job,
    ) -> (String, anyhow::Result<codetether_swarm_core::SubTaskResult>) {
        super::super::job_run::execute(job).await
    }
}
