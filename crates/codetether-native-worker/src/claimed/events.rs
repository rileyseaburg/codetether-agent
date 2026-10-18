use super::Native;
use crate::{
    event_sink::{clear_task_event_sink, install_task_event_sink},
    workspace_scope::sync_timeline_to_runtime,
};
use codetether_a2a_http::worker::output_callback::build_output_callback;
use codetether_a2a_worker_core::task_driver::claimed::{Events, Output};
use codetether_a2a_worker_core::task_timeline::TaskTimeline;
impl<S: Sync, F> Events for Native<'_, S, F> {
    async fn sync(&self, timeline: &TaskTimeline) {
        sync_timeline_to_runtime(timeline, self.runtime).await;
    }
    fn output(&self, task_id: &str) -> Output {
        build_output_callback(
            self.runtime.client.clone(),
            self.runtime.server.clone(),
            self.runtime.token.clone(),
            self.runtime.worker_id.clone(),
            task_id.to_string(),
            self.runtime.bus.clone(),
        )
    }
    fn install(&self, task_id: &str) {
        install_task_event_sink(self.runtime, task_id);
    }
    fn clear(&self) {
        clear_task_event_sink();
    }
}
