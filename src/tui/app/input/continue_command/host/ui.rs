use super::Native;
use crate::tui::app::{input::chat_helpers::push_user_messages, watchdog::WatchdogNotification};
use codetether_chat_model::ChatMessage;
impl codetether_chat_submit::resume::Ui for Native<'_> {
    fn clear_input(&mut self) {
        self.app.state.clear_input();
    }
    fn set_restarts(&mut self, count: u32) {
        self.app.state.main_watchdog_restart_count = count;
    }
    fn set_inflight(&mut self, prompt: String) {
        self.app.state.main_inflight_prompt = Some(prompt);
    }
    fn arm_retry(&mut self, message: String, count: u32) {
        self.app.state.watchdog_notification = Some(WatchdogNotification::new(message, count));
    }
    fn set_processing(&mut self, processing: bool) {
        self.app.state.processing = processing;
    }
    fn clear_streaming(&mut self) {
        self.app.state.clear_streaming_text();
    }
    fn clear_timing(&mut self) {
        self.app.state.clear_request_timing();
    }
    fn status(&mut self, message: String) {
        self.app.state.status = message;
    }
    fn messages_mut(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.app.state.messages
    }
    fn publish_user(&mut self, prompt: &str) {
        push_user_messages(self.app, prompt, &[]);
    }
    fn scroll(&mut self) {
        self.app.state.scroll_to_bottom();
    }
}
