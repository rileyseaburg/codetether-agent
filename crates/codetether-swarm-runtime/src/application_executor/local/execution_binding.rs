//! Native agent execution for local stages.
use super::{LocalStage, LocalStageHost};
impl<H: LocalStageHost> codetether_swarm_local_stage::Execution for LocalStage<'_, H> {
    fn run_job(
        job: codetether_swarm_local_job::Job,
    ) -> impl std::future::Future<
        Output = (String, anyhow::Result<codetether_swarm_core::SubTaskResult>),
    > + Send
    + 'static {
        H::run_job(job)
    }
}
