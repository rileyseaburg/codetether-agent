use crate::WorkerTaskRuntime;
use codetether_a2a_worker_core::dispatch::pending::Host;
pub(super) struct Native<'a, D> {
    pub runtime: &'a WorkerTaskRuntime,
    pub dispatcher: std::marker::PhantomData<fn() -> D>,
}
impl<D: super::Dispatcher> Host for Native<'_, D> {
    async fn pending_tasks(&self) -> anyhow::Result<Vec<serde_json::Value>> {
        codetether_a2a_http::worker::pending_tasks(
            &self.runtime.client,
            &self.runtime.server,
            &self.runtime.token,
            &self.runtime.agent_name,
        )
        .await
    }
    async fn dispatch(&self, task: &serde_json::Value) {
        D::dispatch(task, self.runtime).await;
    }
}
