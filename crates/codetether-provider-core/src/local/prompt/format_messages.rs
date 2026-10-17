//! Local-provider format messages.
use super::*;
impl LocalCudaPrompt {
    /// Format messages into a prompt string
    /// Render the legacy local chat prompt with role prefixes.
    pub fn format_messages(messages: &[Message]) -> String {
        let mut prompt = String::new();

        for msg in messages {
            match msg.role {
                Role::System | Role::Developer => {
                    prompt.push_str("System: ");
                    prompt.push_str(&Self::content_to_string(&msg.content));
                    prompt.push_str("\n\n");
                }
                Role::User => {
                    prompt.push_str("User: ");
                    prompt.push_str(&Self::content_to_string(&msg.content));
                    prompt.push_str("\n\n");
                }
                Role::Assistant => {
                    prompt.push_str("Assistant: ");
                    prompt.push_str(&Self::content_to_string(&msg.content));
                    prompt.push_str("\n\n");
                }
                Role::Tool => {
                    // Tool results go in the conversation as context
                    prompt.push_str("Tool: ");
                    prompt.push_str(&Self::content_to_string(&msg.content));
                    prompt.push_str("\n\n");
                }
            }
        }

        prompt.push_str("Assistant: ");
        prompt
    }
}
