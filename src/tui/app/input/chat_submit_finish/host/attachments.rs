use super::Native;
use crate::session::ImageAttachment;
use crate::tui::app::input::pasted_text::{PendingTextPaste, expand_paste_placeholders};
impl codetether_chat_submit::finish::Attachments for Native<'_> {
    type Image = ImageAttachment;
    type TextPastes = Vec<PendingTextPaste>;
    fn take_images(&mut self) -> Vec<Self::Image> {
        std::mem::take(&mut self.app.state.pending_images)
    }
    fn take_text(&mut self) -> Self::TextPastes {
        std::mem::take(&mut self.app.state.pending_text_pastes)
    }
    fn expand(&self, prompt: &str, text: &Self::TextPastes) -> String {
        expand_paste_placeholders(prompt, text)
    }
}
