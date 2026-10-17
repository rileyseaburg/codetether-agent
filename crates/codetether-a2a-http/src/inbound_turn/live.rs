//! Await an interactive answer while refreshing task liveness.
use super::*;

pub(super) async fn live(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    from: &str,
    prompt: &str,
) -> Result<String> {
    let mut pending = live_inbox::enqueue(task_id, context_id, from, prompt);
    loop {
        match tokio::time::timeout(LIVE_HEARTBEAT, &mut pending).await {
            Ok(outcome) => {
                return outcome.map_err(|reason| anyhow::anyhow!("interactive session: {reason}"));
            }
            Err(_) => touch(tasks, task_id),
        }
    }
}

/// Bump the task's status timestamp so pollers see the turn is alive.
fn touch(tasks: &DashMap<String, Task>, task_id: &str) {
    if let Some(mut task) = tasks.get_mut(task_id) {
        task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
    }
}
