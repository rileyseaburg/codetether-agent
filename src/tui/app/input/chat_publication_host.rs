use super::{App, TuiWorkerBridge};
use crate::tui::app::worker_bridge::handle_processing_stopped;
use codetether_chat_model::ChatMessage;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub bridge: &'a Option<TuiWorkerBridge>,
}
impl codetether_chat_submit::publication::FailureHost for Native<'_> {
    async fn processing_stopped(&mut self) {
        handle_processing_stopped(self.app, self.bridge).await;
    }
    fn clear_timing(&mut self) {
        self.app.state.clear_request_timing();
    }
    fn push(&mut self, message: ChatMessage) {
        self.app.state.messages.push(message);
    }
    fn status(&mut self, message: String) {
        self.app.state.status = message;
    }
    fn scroll(&mut self) {
        self.app.state.scroll_to_bottom();
    }
}
