//! Mutable projection state for the parts of one provider message.
use codetether_chat_model::ChatMessage;
use codetether_provider_core::Role;
use std::collections::HashMap;
pub(crate) struct PartProjection<'a> {
    pub(crate) role: Role,
    pub(crate) names: &'a mut HashMap<String, String>,
    pub(crate) text_buffer: String,
    pub(crate) messages: Vec<ChatMessage>,
}
impl<'a> PartProjection<'a> {
    pub(crate) fn new(role: Role, names: &'a mut HashMap<String, String>) -> Self {
        Self {
            role,
            names,
            text_buffer: String::new(),
            messages: Vec::new(),
        }
    }
    pub(crate) fn flush(&mut self) {
        crate::buffer::flush_text_buffer(self.role, &mut self.text_buffer, &mut self.messages);
    }
    pub(crate) fn finish(mut self) -> Vec<ChatMessage> {
        self.flush();
        self.messages
    }
}
