//! Native bindings for shared working-tree report presentation.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};
use std::path::Path;
impl codetether_git_view::command::View for App {
    fn clear_input(&mut self) {
        self.state.clear_input();
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
    fn push_system(&mut self, text: String) {
        self.state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll_to_bottom(&mut self) {
        self.state.scroll_to_bottom();
    }
}
pub(super) fn show(app: &mut App, cwd: &Path) {
    codetether_git_view::command::show(app, cwd);
}
