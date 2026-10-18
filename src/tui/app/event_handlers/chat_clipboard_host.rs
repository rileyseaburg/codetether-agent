//! Native state bindings for chat clipboard workflows.
use crate::tui::app::state::App;
use codetether_chat_model::ChatMessage;
use codetether_image::ImageAttachment;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_chat_clipboard::Host for Native<'_> {
    fn messages(&self) -> &[ChatMessage] {
        &self.0.state.messages
    }
    fn pending_images(&mut self) -> &mut Vec<ImageAttachment> {
        &mut self.0.state.pending_images
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn scroll_bottom(&mut self) {
        self.0.state.chat_viewport.chat_scroll = crate::tui::constants::SCROLL_BOTTOM;
    }
    fn paste_into_chat(&mut self, text: &str) {
        crate::tui::app::input::paste_into_chat(self.0, text);
    }
}
