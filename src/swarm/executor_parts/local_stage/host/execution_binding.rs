//! Native execution binding for local stages.
use super::Native;
use crate::swarm::{CollapseController, CollapsePolicy};
impl codetether_swarm_local_stage::Execution for Native<'_> {
    type Controller = CollapseController;

    fn controller() -> Self::Controller {
        CollapseController::new(CollapsePolicy::default())
    }

    fn sample(
        controller: &mut Self::Controller,
        branches: &[codetether_swarm_core::collapse::BranchRuntimeState],
    ) -> anyhow::Result<codetether_swarm_core::collapse::CollapseTick> {
        controller.sample(branches)
    }

    async fn run_job(
        job: codetether_swarm_local_job::Job,
    ) -> (String, anyhow::Result<codetether_swarm_core::SubTaskResult>) {
        super::super::job_run::execute(job).await
    }
}
