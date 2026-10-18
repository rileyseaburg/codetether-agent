//! Native agent execution for local stages.
use super::{LocalStage, LocalStageHost};
impl<H: LocalStageHost> codetether_swarm_local_stage::Execution for LocalStage<'_, H> {
    async fn run_job(
        job: codetether_swarm_local_job::Job,
    ) -> (String, anyhow::Result<codetether_swarm_core::SubTaskResult>) {
        H::run_job(job).await
    }
}
