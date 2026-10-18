//! Provider-load failure settlement for a streaming task.
use codetether_a2a_core::{Task, TaskState};
use dashmap::DashMap;
use std::future::Future;

/// Load providers or mark the streaming task failed.
///
/// # Arguments
/// * `tasks` — Shared task records.
/// * `task_id` — Task awaiting provider initialization.
/// * `load` — Application-owned provider-loading future.
/// # Returns
/// The loaded registry, or `None` after logging and recording load failure.
/// # Examples
/// ```no_run
/// # async fn example(tasks: &dashmap::DashMap<String, codetether_a2a_core::Task>) {
/// let registry = codetether_a2a_http::task_state::stream::registry::load(
///     tasks, "task", async { Ok::<_, anyhow::Error>(()) },
/// ).await;
/// assert!(registry.is_some());
/// # }
/// ```
pub async fn load<R, F>(tasks: &DashMap<String, Task>, task_id: &str, load: F) -> Option<R>
where
    F: Future<Output = anyhow::Result<R>>,
{
    match load.await {
        Ok(r) => Some(r),
        Err(e) => {
            tracing::error!(target: "codetether_agent::a2a::server", "Failed to load provider registry: {}", e);
            if let Some(mut t) = tasks.get_mut(task_id) {
                t.status.state = TaskState::Failed;
                t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
            }
            None
        }
    }
}
