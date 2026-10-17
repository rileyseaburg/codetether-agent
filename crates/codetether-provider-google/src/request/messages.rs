//! Apply shared image preservation around Google role encoding.
use crate::*;
impl GoogleProvider {
    /// Convert stored messages while preserving images and thought signatures.
    ///
    /// # Arguments
    /// * messages - Conversation messages in order.
    ///
    /// # Returns
    /// Google-compatible Chat JSON, including image companion messages.
    ///
    /// # Examples
    ///
    ///     assert!(codetether_provider_google::GoogleProvider::convert_messages(&[]).is_empty());
    pub fn convert_messages(messages: &[Message]) -> Vec<Value> {
        codetether_provider_core::chat_images::convert(messages, |msg| {
            let role = match msg.role {
                Role::System | Role::Developer => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Tool => "tool",
            };

            if msg.role == Role::Tool {
                return super::tool::convert(msg);
            }
            if msg.role == Role::Assistant {
                return super::assistant::convert(msg);
            }
            super::ordinary::convert(msg, role)
        })
    }
}
