//! Application progress synchronization and shared workspace routing.
use super::{WorkerTaskRuntime, task_timeline};
pub use codetether_a2a_http::workspace::resolve_and_log_workspace_ids;

pub async fn sync_timeline_to_runtime(
    timeline: &task_timeline::TaskTimeline,
    runtime: &WorkerTaskRuntime,
) {
    timeline.sync_progress().await;
    *runtime.task_progress.lock().await = timeline.progress_handle().lock().await.clone();
}
