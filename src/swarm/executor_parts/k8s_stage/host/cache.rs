//! Preserve cache operations beneath the executor's existing mutex.
use crate::swarm::{SubTask, SubTaskResult, SwarmCache};
impl codetether_swarm_k8s::Cache for SwarmCache {
    async fn get(&mut self, task: &SubTask) -> Option<SubTaskResult> {
        SwarmCache::get(self, task).await
    }
    async fn put(&mut self, task: &SubTask, result: &SubTaskResult) -> anyhow::Result<()> {
        SwarmCache::put(self, task, result).await
    }
}
