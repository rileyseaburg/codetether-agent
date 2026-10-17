//! Chat image and role conversion.
use crate::*;
impl CopilotProvider {
    /// Convert messages into Copilot JSON, preserving images.
    ///
    /// # Arguments
    /// * `messages` - Conversation in provider-neutral form.
    ///
    /// # Returns
    /// Ordered JSON messages.
    ///
    /// # Examples
    /// ```
    /// assert!(codetether_provider_copilot::CopilotProvider::convert_messages(&[]).is_empty());
    /// ```
    pub fn convert_messages(messages: &[Message]) -> Vec<Value> {
        codetether_provider_core::chat_images::convert(messages, |msg| {
            let role = match msg.role {
                Role::System | Role::Developer => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Tool => "tool",
            };

            match msg.role {
                Role::Tool => tool::encode(msg, role),
                Role::Assistant => assistant::encode(msg),
                _ => text::encode(msg, role),
            }
        })
    }
}
mod assistant;
mod body;
mod flags;
mod text;
mod tool;
mod tools;
