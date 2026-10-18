//! Native transcript, worker and presentation operations.
use super::{Native, Session};
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;
impl codetether_tui_startup::hydrate::Effects<Session> for Native<'_> {
    type Worker = TuiWorkerBridge;
    fn sync_messages(&mut self, session: &Session, source: &str, older: bool) {
        crate::tui::app::message_text::sync_messages_from_source(self.0, session, source, older);
    }
    fn attach_worker(&mut self, worker: Option<&TuiWorkerBridge>) {
        if let Some(bridge) = worker {
            self.0
                .state
                .worker
                .set_worker_bridge(bridge.worker_id.clone(), bridge.worker_name.clone());
            self.0.state.worker.register_worker_agent("tui".to_string());
        }
    }
    fn refresh_suggestions(&mut self) {
        self.0.state.composer.refresh_slash_suggestions();
    }
    fn move_cursor_end(&mut self) {
        self.0.state.composer.move_cursor_end();
    }
    fn push_system(&mut self, text: String) {
        self.0
            .state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.chat_viewport.scroll_to_bottom();
    }
    fn policy_summary(&self) -> Option<String> {
        crate::tui::ui::trust_status::startup_summary()
    }
}
