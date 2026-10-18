//! Native task queue and bus access for shared HTTP routes.
use crate::AppState;
impl<R: Send + Sync + 'static> codetether_task_api::http::HttpState for AppState<R> {
    fn queue(&self) -> &codetether_task_queue::KnativeTaskQueue {
        &self.inner.knative_tasks
    }
    fn bus(&self) -> &std::sync::Arc<codetether_bus_core::AgentBus> {
        &self.inner.bus
    }
}

impl<R: Send + Sync + 'static> codetether_task_api::worker_http::WorkerState for AppState<R> {
    fn into_queue(self) -> codetether_task_queue::KnativeTaskQueue {
        self.inner.knative_tasks
    }
}
