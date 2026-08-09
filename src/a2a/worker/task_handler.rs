//! Claimed task handling and release orchestration.

use std::{future::Future, time::Duration};

use anyhow::{Result, anyhow};

use super::{WorkerTaskRuntime, task_metadata, task_str, task_timeout_secs};
use super::{execute_claimed_task, sync_timeline_to_runtime};
use super::{task_claim::claim_task, task_handler_release, task_outcome, task_timeline};

pub(super) async fn handle_task(
    runtime: &WorkerTaskRuntime,
    task: &serde_json::Value,
) -> Result<()> {
    let task_id = task_str(task, "id").ok_or_else(|| anyhow::anyhow!("No task ID"))?;
    let title = task_str(task, "title").unwrap_or("Untitled");
    let timeout_secs = timeout_secs(task);
    let mut timeline = task_timeline::TaskTimeline::new(task_id, timeout_secs);
    timeline.checkpoint(task_timeline::TaskCheckpoint::TaskReceived);
    tracing::info!(task_id, title, "Handling task");
    sync_timeline_to_runtime(&timeline, runtime).await;

    let Some(claimed) = claim_task(runtime, task_id, &mut timeline).await? else {
        return Ok(());
    };
    // The claim response may lower or raise the task budget. Enforce the
    // remaining claim-adjusted budget rather than the pre-claim payload value.
    let execution_timeout = Duration::from_secs_f64(timeline.remaining_secs().max(0.001));
    let inner = execute_with_timeout(
        task_id,
        execution_timeout,
        execute_claimed_task(
            runtime,
            task,
            task_id,
            title,
            &claimed.claim_provenance,
            claimed.provider_keys,
            &mut timeline,
        ),
    )
    .await;
    let outcome = task_outcome::from_inner(task_id, inner, &mut timeline);
    task_handler_release::release_handled_task(runtime, task_id, outcome, &mut timeline).await
}

async fn execute_with_timeout<F, T>(task_id: &str, timeout: Duration, future: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    tokio::time::timeout(timeout, future).await.map_err(|_| {
        anyhow!(
            "Task {task_id} execution exceeded {} seconds",
            timeout.as_secs()
        )
    })?
}

fn timeout_secs(task: &serde_json::Value) -> u64 {
    let metadata = task_metadata(task);
    task_timeout_secs(task, &metadata)
}

#[cfg(test)]
mod tests {
    use std::{future::pending, time::Duration};

    use super::execute_with_timeout;

    #[tokio::test]
    async fn returns_completed_execution_result() {
        let result = execute_with_timeout("task-ok", Duration::from_secs(1), async {
            Ok::<_, anyhow::Error>("done")
        })
        .await;

        assert_eq!(result.expect("execution should complete"), "done");
    }

    #[tokio::test]
    async fn cancels_execution_after_task_budget() {
        let result = execute_with_timeout(
            "task-stuck",
            Duration::from_millis(10),
            pending::<anyhow::Result<()>>(),
        )
        .await;

        let error = result.expect_err("execution should time out").to_string();
        assert!(error.contains("task-stuck"));
        assert!(error.contains("execution exceeded"));
    }
}
