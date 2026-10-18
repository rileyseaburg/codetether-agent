use crate::a2a::worker::{WorkerTaskRuntime, task_dispatch::spawn_task_handler};
use codetether_a2a_worker_core::dispatch::pending::Host;
pub(super) struct Native<'a>(pub &'a WorkerTaskRuntime);
impl Host for Native<'_> {
    async fn pending_tasks(&self) -> anyhow::Result<Vec<serde_json::Value>> {
        codetether_a2a_http::worker::pending_tasks(
            &self.0.client,
            &self.0.server,
            &self.0.token,
            &self.0.agent_name,
        )
        .await
    }
    async fn dispatch(&self, task: &serde_json::Value) {
        spawn_task_handler(task, self.0).await;
    }
}
