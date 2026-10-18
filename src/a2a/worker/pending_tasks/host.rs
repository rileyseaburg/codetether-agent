//! Native claim-dispatch hook for shared pending-task intake.
use crate::a2a::worker::{WorkerTaskRuntime, task_dispatch::spawn_task_handler};
pub(in crate::a2a::worker) struct Native;
impl codetether_native_worker::pending::Dispatcher for Native {
    async fn dispatch(task: &serde_json::Value, runtime: &WorkerTaskRuntime) {
        spawn_task_handler(task, runtime).await;
    }
}
