//! Bind the shared chat shell lifecycle to the native frontend.
use crate::tui::app::state::App;
use codetether_chat_model::ChatMessage;
impl codetether_chat_shell::View for App {
    fn shell(&mut self) -> &mut codetether_chat_shell::State {
        &mut self.state.shell
    }
    fn push_message(&mut self, message: ChatMessage) {
        self.state.messages.push(message);
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
    fn clear_input(&mut self) {
        self.state.clear_input();
    }
    fn scroll_to_bottom(&mut self) {
        self.state.scroll_to_bottom();
    }
}
