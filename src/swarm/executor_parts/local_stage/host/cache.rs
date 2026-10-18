//! Preserve the existing cache lock scope for stage lookups and writes.
use super::SwarmExecutor;
use codetether_swarm_core::{SubTask, SubTaskResult};
pub(super) async fn get(executor: &SwarmExecutor, task: &SubTask) -> Option<SubTaskResult> {
    match &executor.cache {
        Some(cache) => cache.lock().await.get(task).await,
        None => None,
    }
}
pub(super) async fn put(
    executor: &SwarmExecutor,
    task: &SubTask,
    result: &SubTaskResult,
) -> anyhow::Result<()> {
    match &executor.cache {
        Some(cache) => cache.lock().await.put(task, result).await,
        None => Ok(()),
    }
}
