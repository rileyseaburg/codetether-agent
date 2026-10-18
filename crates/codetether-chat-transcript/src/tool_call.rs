//! Remember tool identities and project retained call arguments.
use crate::PartProjection;
use codetether_chat_model::{ChatMessage, MessageType, payload};
use codetether_util::preview_chars::truncate_preview;
impl PartProjection<'_> {
    pub(crate) fn tool_call(&mut self, id: &str, name: &str, arguments: &str) {
        self.flush();
        self.names.insert(id.to_owned(), name.to_owned());
        self.messages.push(ChatMessage::new(
            MessageType::ToolCall {
                name: name.to_owned(),
                arguments: payload::tool_arguments(arguments),
            },
            format!("{name}: {}", truncate_preview(arguments, 240)),
        ));
    }
}
