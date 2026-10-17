//! History and tool encoding with shared image preservation.
use crate::*;
mod assistant;
mod content;
mod ordinary;
mod tool;
mod tools;
impl OpenRouterProvider {
    /// Encode history with the existing image and tool-result conventions.
    ///
    /// # Arguments
    /// * messages - Conversation messages in order.
    ///
    /// # Returns
    /// OpenRouter-compatible Chat message objects.
    ///
    /// # Examples
    ///
    ///     assert!(codetether_provider_openrouter::OpenRouterProvider::convert_messages(&[]).is_empty());
    pub fn convert_messages(messages: &[Message]) -> Vec<Value> {
        codetether_provider_core::chat_images::convert(messages, |msg| {
            let role = match msg.role {
                Role::System | Role::Developer => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Tool => "tool",
            };

            match msg.role {
                Role::Tool => tool::convert(msg, role),
                Role::Assistant => assistant::convert(msg),
                _ => ordinary::convert(msg, role),
            }
        })
    }
}
