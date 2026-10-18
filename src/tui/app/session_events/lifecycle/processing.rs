//! Worker notification and startup timing adapters.
use super::Native;
impl codetether_session_events::lifecycle::Processing for Native<'_> {
    async fn processing_started(&mut self) {
        crate::tui::app::worker_bridge::handle_processing_started(self.app, self.worker_bridge)
            .await;
    }
    async fn processing_stopped(&mut self) {
        crate::tui::app::worker_bridge::handle_processing_stopped(self.app, self.worker_bridge)
            .await;
    }
    fn request_timing_started(&self) -> bool {
        self.app
            .state
            .request_timing
            .processing_started_at
            .is_some()
    }
    fn begin_request_timing(&mut self) {
        self.app.state.begin_request_timing();
    }
    fn streaming_started(&self) -> bool {
        self.app.state.streaming.streaming_start.is_some()
    }
    fn begin_streaming(&mut self) {
        self.app.state.begin_streaming();
    }
    fn set_status(&mut self, status: String) {
        self.app.state.status = status;
    }
}
