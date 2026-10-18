use super::{App, Session, SessionSlot, TuiWorkerBridge};
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub slot: &'a mut SessionSlot,
    pub worker_bridge: &'a mut Option<TuiWorkerBridge>,
}
impl codetether_session_events::completion::failure::Host for Native<'_> {
    type Session = Session;
    fn restore(&mut self, session: Session) {
        self.slot.restore(session);
    }
    async fn stopped(&mut self) {
        crate::tui::app::worker_bridge::handle_processing_stopped(self.app, self.worker_bridge)
            .await;
    }
    fn complete_timing(&mut self) {
        self.app.state.request_timing.complete_request_timing();
    }
    fn reconnect(&mut self, error: &str) {
        crate::tui::app::background::stream_reconnect::schedule(self.app, error);
    }
    fn smart_switch(&mut self, error: &str) {
        super::super::super::runtime_retry::schedule(self.app, self.slot, error);
    }
    fn error(&mut self, error: String) {
        self.app
            .state
            .messages
            .push(ChatMessage::new(MessageType::Error, error));
    }
    fn status(&mut self) -> &mut String {
        &mut self.app.state.status
    }
    fn scroll(&mut self) {
        self.app.state.scroll_to_bottom();
    }
}
