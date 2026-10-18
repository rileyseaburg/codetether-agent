//! Native session and provider context for remote prompt dispatch.
use super::*;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub slot: &'a mut SessionSlot,
    pub registry: &'a Option<Arc<ProviderRegistry>>,
    pub worker_bridge: &'a Option<TuiWorkerBridge>,
    pub runtime: &'a TuiSessionHandle,
}
impl codetether_a2a_interactive::Presentation for Native<'_> {
    fn push_system(&mut self, text: String) {
        codetether_a2a_interactive::Presentation::push_system(self.app, text);
    }
    fn scroll_to_bottom(&mut self) {
        self.app.state.scroll_to_bottom();
    }
    fn set_status(&mut self, status: String) {
        self.app.state.status = status;
    }
}
impl codetether_a2a_interactive::Dispatch for Native<'_> {
    fn processing(&self) -> bool {
        self.app.state.processing
    }
    fn dequeue_task(&mut self) -> Option<IncomingTask> {
        self.app.state.dequeue_worker_task()
    }
    fn set_active_task(&mut self, task: IncomingTask) {
        self.app.state.worker.active_remote_task = Some(task);
    }
    async fn dispatch_prompt(&mut self, prompt: &str) {
        dispatch_prompt(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            prompt,
            Vec::new(),
            self.runtime,
        )
        .await;
    }
}
