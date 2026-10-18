//! Native task queue and bus access for shared HTTP routes.
impl codetether_task_api::http::HttpState for super::AppState {
    fn queue(&self) -> &codetether_task_queue::KnativeTaskQueue {
        &self.knative_tasks
    }
    fn bus(&self) -> &std::sync::Arc<codetether_bus_core::AgentBus> {
        &self.bus
    }
}
