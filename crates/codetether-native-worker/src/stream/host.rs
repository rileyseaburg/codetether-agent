use super::request::build_stream_request;
use crate::{
    WorkerTaskRuntime,
    pending::{Dispatcher, poll_pending_tasks},
};
use codetether_a2a_http::worker::connect_error;
use codetether_a2a_stream::{event_id::EventId, worker_connection::Host};
use codetether_worker_http::WorkerServerState;
pub(super) struct Native<'a, D> {
    pub runtime: &'a WorkerTaskRuntime,
    pub dispatcher: std::marker::PhantomData<fn() -> D>,
    pub name: &'a str,
    pub codebases: &'a [String],
    pub server_state: Option<&'a WorkerServerState>,
}
impl<D: Dispatcher> codetether_a2a_stream::dispatch::Host for Native<'_, D> {
    async fn dispatch(&self, task: &serde_json::Value) {
        D::dispatch(task, self.runtime).await;
    }
    async fn reconcile(&self) -> anyhow::Result<()> {
        poll_pending_tasks::<D>(self.runtime).await
    }
}
impl<D: Dispatcher> Host for Native<'_, D> {
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
