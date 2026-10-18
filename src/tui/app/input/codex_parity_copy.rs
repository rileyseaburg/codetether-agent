//! Native copy command entry and view bindings.
use crate::tui::app::state::App;
use codetether_chat_model::ChatMessage;
impl codetether_chat_clipboard::command::View for App {
    fn messages(&self) -> &[ChatMessage] {
        &self.state.messages
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
    fn clear_input(&mut self) {
        self.state.clear_input();
    }
}
pub(super) fn run(app: &mut App, prompt: &str) {
    codetether_chat_clipboard::command::run(app, prompt);
}
#[cfg(test)]
#[path = "codex_parity_copy_tests.rs"]
mod tests;
