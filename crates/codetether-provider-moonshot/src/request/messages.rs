//! Decorate role serialization with shared image preservation.
use crate::*;
impl MoonshotProvider {
    /// Encode Moonshot history while retaining shared image handling.
    ///
    /// # Arguments
    /// * messages - Stored conversation in order.
    ///
    /// # Returns
    /// JSON messages with the existing role, reasoning, and tool-call rules.
    ///
    /// # Examples
    ///
    ///     use codetether_provider_moonshot::MoonshotProvider;
    ///     assert!(MoonshotProvider::convert_messages(&[]).is_empty());
    pub fn convert_messages(messages: &[Message]) -> Vec<Value> {
        codetether_provider_core::chat_images::convert(messages, |msg| {
            let role = match msg.role {
                Role::System | Role::Developer => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Tool => "tool",
            };

            match msg.role {
                Role::Tool => super::tool::convert(msg, role),
                Role::Assistant => super::assistant::convert(msg),
                _ => super::ordinary::convert(msg, role),
            }
        })
    }
}
