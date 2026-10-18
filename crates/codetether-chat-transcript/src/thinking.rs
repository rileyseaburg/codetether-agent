//! Thinking rows flush any preceding plain text.
use crate::PartProjection;
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_util::preview_chars::truncate_preview;
impl PartProjection<'_> {
    pub(crate) fn thinking(&mut self, text: &str) {
        self.flush();
        self.messages.push(ChatMessage::new(
            MessageType::Thinking(text.to_owned()),
            truncate_preview(text, 600),
        ));
    }
}
