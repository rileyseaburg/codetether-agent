use crate::a2a::worker::{
    WorkerTaskRuntime, pending_tasks::poll_pending_tasks, task_dispatch::spawn_task_handler,
    task_stream_error::connect_error, task_stream_request::build_stream_request,
};
use crate::worker_server::WorkerServerState;
use codetether_a2a_stream::{event_id::EventId, worker_connection::Host};
pub(super) struct Native<'a> {
    pub runtime: &'a WorkerTaskRuntime,
    pub name: &'a str,
    pub codebases: &'a [String],
    pub server_state: Option<&'a WorkerServerState>,
}
impl codetether_a2a_stream::dispatch::Host for Native<'_> {
    async fn dispatch(&self, task: &serde_json::Value) {
        spawn_task_handler(task, self.runtime).await;
    }
    async fn reconcile(&self) -> anyhow::Result<()> {
        poll_pending_tasks(self.runtime).await
    }
}
impl Host for Native<'_> {
    fn request(&self, last: Option<&EventId>) -> reqwest::RequestBuilder {
        build_stream_request(self.runtime, self.name, self.codebases, last)
    }
    async fn connected(&self) {
        if let Some(state) = self.server_state {
            state.set_connected(true).await;
        }
    }
    async fn connect_error(&self, response: reqwest::Response) -> anyhow::Error {
        connect_error(response).await
    }
}
