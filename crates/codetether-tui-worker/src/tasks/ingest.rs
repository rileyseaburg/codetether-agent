use crate::{IncomingTask, WorkerState};
use tokio::sync::mpsc;

/// Drain ready tasks into the bounded worker queue and update the status.
///
/// # Arguments
/// * `state` - Worker state owning the queue and recent-task history.
/// * `status` - UI status updated after each accepted task.
/// * `receiver` - Bridge receiver to drain without waiting.
/// # Returns
/// Returns after all currently available tasks have been queued.
/// # Examples
/// ```
/// use codetether_tui_worker::{tasks, WorkerState};
/// let (_, mut receiver) = tokio::sync::mpsc::channel(1);
/// let mut state = WorkerState::default();
/// let mut status = String::new();
/// tasks::ingest(&mut state, &mut status, &mut receiver);
/// assert!(state.worker_task_queue.is_empty());
/// ```
pub fn ingest(
    state: &mut WorkerState,
    status: &mut String,
    receiver: &mut mpsc::Receiver<IncomingTask>,
) {
    while let Ok(task) = receiver.try_recv() {
        let preview = format!(
            "{} from {}",
            task.task_id,
            task.from_agent
                .clone()
                .unwrap_or_else(|| "unknown".to_string())
        );
        state.push_recent_task(preview);
        state.enqueue_worker_task(task);
        *status = format!("Queued remote task(s): {}", state.worker_task_queue.len());
    }
}
