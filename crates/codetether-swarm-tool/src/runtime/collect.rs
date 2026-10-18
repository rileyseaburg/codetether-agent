//! Join workers in task order and integrate their successful worktrees.
use super::pending::Pending;
use crate::{task_result::TaskResult, worktrees::SwarmWorktrees};
pub(super) async fn join(
    handles: Vec<Pending>,
    worktrees: &SwarmWorktrees,
) -> (Vec<TaskResult>, usize) {
    let mut results = Vec::new();
    let mut failures = 0;
    for (idx, failed_task_id, failed_task_name, model, handle) in handles {
        match handle.join().await {
            Ok(Ok(mut result)) => {
                worktrees.finish(idx, &mut result).await;
                if !result.success {
                    failures += 1;
                }
                results.push(result);
            }
            Ok(Err(e)) => {
                failures += 1;
                tracing::error!(error = %e, "Task execution failed");
                results.push(TaskResult::failed(
                    failed_task_id,
                    failed_task_name,
                    e.to_string(),
                    model,
                ));
            }
            Err(e) => {
                failures += 1;
                tracing::error!(error = %e, "Task join failed");
                results.push(TaskResult::failed(
                    failed_task_id,
                    failed_task_name,
                    format!("Task join failed: {e}"),
                    model,
                ));
            }
        }
    }
    (results, failures)
}
