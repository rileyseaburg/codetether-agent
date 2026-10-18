//! Native composer state operations used by shared file attachments.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
    models::{InputMode, ViewMode},
};
use std::path::Path;
pub(super) struct Composer<'a>(pub(super) &'a mut App);
impl codetether_file_share::interactive::View for Composer<'_> {
    fn input(&mut self) -> &mut String {
        &mut self.0.state.composer.input
    }
    fn set_cursor(&mut self, cursor: usize) {
        self.0.state.composer.cursor = cursor;
    }
    fn start_editing(&mut self) {
        self.0.state.composer.mode = InputMode::Editing;
    }
    fn clear_history_selection(&mut self) {
        self.0.state.composer.history.history_index = None;
    }
    fn refresh_suggestions(&mut self) {
        self.0.state.composer.refresh_slash_suggestions();
    }
    fn show_chat(&mut self) {
        self.0.state.set_view_mode(ViewMode::Chat);
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.chat_viewport.scroll_to_bottom();
    }
    fn notice(&mut self, message: String, status: &str) {
        self.0
            .state
            .messages
            .push(ChatMessage::new(MessageType::System, message));
        self.0.state.status = status.to_string();
        self.0.state.chat_viewport.scroll_to_bottom();
    }
    fn open_picker(&mut self, workspace: &Path) {
        crate::tui::app::file_picker::open_file_picker(self.0, workspace);
    }
}
