//! Native inbox, failover, timing and message effects on error.
use super::Native;
use codetether_chat_model::ChatMessage;
impl codetether_chat_events::error::View for Native<'_> {
    fn settle_error(&mut self, error: String) {
        crate::tui::app::inbox::live::settle(self.app, Err(error));
    }
    fn complete_request_timing(&mut self) {
        self.app.state.complete_request_timing();
    }
    fn schedule_retry(&mut self, error: &str) {
        super::smart_switch::schedule(self.app, self.slot, error);
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.app.state.messages
    }
    fn auto_follow(&self) -> bool {
        self.app.state.chat_viewport.chat_auto_follow
    }
    fn scroll_to_bottom(&mut self) {
        self.app.state.scroll_to_bottom();
    }
}
