//! Native pending-image queue and output bindings for shared attachment policy.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};
use codetether_image::{
    ImageAttachment,
    interactive::{Presentation, View},
};
pub(crate) use codetether_image::{attachment_from_file as attach_image_file, image_summary};
use std::path::Path;
struct ImageView<'a>(&'a mut App);
impl View for ImageView<'_> {
    fn push_image(&mut self, image: ImageAttachment) -> usize {
        self.0.state.pending_images.push(image);
        self.0.state.pending_images.len()
    }
    fn set_status(&mut self, message: String) {
        self.0.state.status = message;
    }
    fn push_system_message(&mut self, message: String) {
        self.0
            .state
            .messages
            .push(ChatMessage::new(MessageType::System, message));
        self.0.state.scroll_to_bottom();
    }
}
pub(crate) fn attach_image_command(app: &mut App, cwd: &Path, cleaned: &str, mode: Presentation) {
    codetether_image::interactive::attach(&mut ImageView(app), cwd, cleaned, mode);
}
