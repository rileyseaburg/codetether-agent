use super::{ProcessingSet, RuntimeConfig, WorkerTaskRuntime};
use crate::task_timeline::TaskProgressState;
use std::sync::Arc;
use tokio::sync::Mutex;
/// Construct shared task runtime state from startup settings and services.
/// # Arguments
/// `client` is cloned; `settings` supplies identity and policies; `processing`,
/// `bus`, and `progress` share state across task handlers.
/// # Returns
/// Runtime state with normalized concurrency and parsed approval policy.
/// # Examples
/// ```no_run
/// # fn build<C: Clone, B>(client: &C, bus: &std::sync::Arc<B>, progress: &std::sync::Arc<tokio::sync::Mutex<codetether_a2a_worker_core::task_timeline::TaskProgressState>>) {
/// use codetether_a2a_worker_core::initialization::{build_task_runtime, processing_set, RuntimeConfig};
/// let settings = RuntimeConfig { server: "http://server", token: None,
///     worker_id: "worker", name: "agent", max_concurrent_tasks: 1, auto_approve: "safe" };
/// let runtime = build_task_runtime(client, settings, &processing_set(), bus, progress);
/// assert_eq!(runtime.worker_id, "worker");
/// # }
/// ```
pub fn build_task_runtime<C: Clone, B>(
    client: &C,
    settings: RuntimeConfig<'_>,
    processing: &ProcessingSet,
    bus: &Arc<B>,
    progress: &Arc<Mutex<TaskProgressState>>,
) -> WorkerTaskRuntime<C, B> {
    WorkerTaskRuntime {
        client: client.clone(),
        server: settings.server.to_string(),
        token: settings.token.map(str::to_owned),
        worker_id: settings.worker_id.to_string(),
        agent_name: settings.name.to_string(),
        processing: processing.clone(),
        max_concurrent_tasks: crate::normalize_max_concurrent_tasks(settings.max_concurrent_tasks),
        auto_approve: crate::execution_policy::parse_auto_approve(settings.auto_approve),
        bus: bus.clone(),
        task_progress: progress.clone(),
        workspace_ids: Vec::new(),
    }
}
