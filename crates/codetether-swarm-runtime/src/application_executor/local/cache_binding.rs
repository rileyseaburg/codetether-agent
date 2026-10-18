//! Native cache binding for local stages.
use super::{LocalStage, LocalStageHost, cache};
impl<H: LocalStageHost> codetether_swarm_local_stage::Cache for LocalStage<'_, H> {
    async fn cache_get(
        &self,
        task: &codetether_swarm_core::SubTask,
    ) -> Option<codetether_swarm_core::SubTaskResult> {
        cache::get(self.0, task).await
    }

    async fn cache_put(
        &self,
        task: &codetether_swarm_core::SubTask,
        result: &codetether_swarm_core::SubTaskResult,
    ) -> anyhow::Result<()> {
        cache::put(self.0, task, result).await
    }
}
