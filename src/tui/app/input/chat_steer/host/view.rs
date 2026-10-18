use super::Native;
use codetether_chat_model::ChatMessage;
impl codetether_chat_submit::steering::View for Native<'_> {
    fn clear_attachments(&mut self) {
        self.app.state.pending_images.clear();
        self.app.state.pending_text_pastes.clear();
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.app.state.messages
    }
    fn clear_input(&mut self) {
        self.app.state.clear_input();
    }
    fn status(&mut self, message: String) {
        self.app.state.status = message;
    }
    fn scroll(&mut self) {
        self.app.state.scroll_to_bottom();
    }
}
