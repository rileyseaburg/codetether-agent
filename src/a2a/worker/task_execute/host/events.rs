use super::Native;
use crate::a2a::worker::{
    build_output_callback, sync_timeline_to_runtime,
    task_execute_sink::{clear_task_event_sink, install_task_event_sink},
    task_timeline::TaskTimeline,
};
use codetether_a2a_worker_core::task_driver::claimed::{Events, Output};
impl Events for Native<'_> {
    async fn sync(&self, timeline: &TaskTimeline) {
        sync_timeline_to_runtime(timeline, self.0).await;
    }
    fn output(&self, task_id: &str) -> Output {
        build_output_callback(
            self.0.client.clone(),
            self.0.server.clone(),
            self.0.token.clone(),
            self.0.worker_id.clone(),
            task_id.to_string(),
            self.0.bus.clone(),
        )
    }
    fn install(&self, task_id: &str) {
        install_task_event_sink(self.0, task_id);
    }
    fn clear(&self) {
        clear_task_event_sink();
    }
}
