//! Role dispatch retaining stored message order.
use crate::*;
impl StepFunProvider {
    pub(crate) fn convert_text_messages(&self, messages: &[Message]) -> Vec<ChatMessage> {
        let mut result = Vec::new();
        for msg in messages {
            match msg.role {
                Role::System | Role::Developer => super::system::push(msg, &mut result),
                Role::User => super::user::push(msg, &mut result),
                Role::Assistant => super::assistant::push(msg, &mut result),
                Role::Tool => super::tool::push(msg, &mut result),
            }
        }
        result
    }
}
