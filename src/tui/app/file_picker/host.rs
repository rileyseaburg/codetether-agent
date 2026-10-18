//! Native file-picker state and composer attachment bindings.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
    models::ViewMode,
};
use codetether_file_picker::FilePickerState;
use codetether_image::ImageAttachment;
use std::path::Path;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_file_picker::interactive::View for Native<'_> {
    fn picker(&mut self) -> &mut FilePickerState {
        &mut self.0.state.file_picker
    }
    fn show_picker(&mut self) {
        self.0.state.view_mode = ViewMode::FilePicker;
    }
    fn show_chat(&mut self) {
        self.0.state.view_mode = ViewMode::Chat;
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn push_image(&mut self, image: ImageAttachment) {
        self.0.state.pending_images.push(image);
    }
    fn push_system(&mut self, text: String) {
        self.0
            .state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn push_error(&mut self, text: String) {
        self.0
            .state
            .messages
            .push(ChatMessage::new(MessageType::Error, text));
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.chat_viewport.scroll_to_bottom();
    }
    fn attach_file(&mut self, workspace: &Path, path: &Path) {
        crate::tui::app::file_share::attach_file_to_input(self.0, workspace, path);
    }
}
