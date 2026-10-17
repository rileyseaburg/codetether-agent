//! Local-provider to prompts.
use super::*;
impl LocalCudaPrompt {
    /// Project message history into separate system and conversation prompts.
    pub fn to_prompts(messages: &[Message]) -> (String, String) {
        let mut system_lines = Vec::new();
        let mut convo_lines = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System | Role::Developer => {
                    let text = Self::content_to_string(&msg.content);
                    if !text.is_empty() {
                        system_lines.push(text);
                    }
                }
                Role::User => {
                    convo_lines.push(format!("User:\n{}", Self::content_to_string(&msg.content)));
                }
                Role::Assistant => {
                    convo_lines.push(format!(
                        "Assistant:\n{}",
                        Self::content_to_string(&msg.content)
                    ));
                }
                Role::Tool => {
                    convo_lines.push(format!("Tool:\n{}", Self::content_to_string(&msg.content)));
                }
            }
        }

        let system_prompt = if system_lines.is_empty() {
            "You are CodeTether local CUDA coding assistant.".to_string()
        } else {
            system_lines.join("\n\n")
        };
        let user_prompt = if convo_lines.is_empty() {
            "User:\n(Empty prompt)".to_string()
        } else {
            convo_lines.join("\n\n")
        };

        (system_prompt, user_prompt)
    }
}
