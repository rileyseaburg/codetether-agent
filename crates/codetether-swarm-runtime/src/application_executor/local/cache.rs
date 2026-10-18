//! Preserve the existing cache lock scope for stage lookups and writes.
use super::LocalStageHost;
use crate::SwarmExecutor;
use codetether_swarm_core::{SubTask, SubTaskResult};
pub(super) async fn get<H: LocalStageHost>(
    executor: &SwarmExecutor<H>,
    task: &SubTask,
) -> Option<SubTaskResult> {
    match &executor.inner.cache {
        Some(cache) => cache.lock().await.get(task).await,
        None => None,
    }
}
pub(super) async fn put<H: LocalStageHost>(
    executor: &SwarmExecutor<H>,
    task: &SubTask,
    result: &SubTaskResult,
) -> anyhow::Result<()> {
    match &executor.inner.cache {
        Some(cache) => cache.lock().await.put(task, result).await,
        None => Ok(()),
    }
}
