//! Image and file rows preserve their original location text.
use crate::PartProjection;
use codetether_chat_model::{ChatMessage, MessageType};
impl PartProjection<'_> {
    pub(crate) fn image(&mut self, url: &str) {
        self.flush();
        self.messages.push(ChatMessage::new(
            MessageType::Image {
                url: url.to_owned(),
            },
            url.to_owned(),
        ));
    }
    pub(crate) fn file(&mut self, path: &str) {
        self.flush();
        self.messages.push(ChatMessage::new(
            MessageType::File {
                path: path.to_owned(),
                size: None,
            },
            path.to_owned(),
        ));
    }
}
