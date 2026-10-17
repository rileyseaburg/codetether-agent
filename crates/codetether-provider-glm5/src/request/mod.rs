//! Message role encoding and request settings.
use crate::*;
impl Glm5Provider {
    pub(crate) fn convert_messages(messages: &[Message]) -> Vec<Value> {
        messages
            .iter()
            .map(|msg| {
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
            .collect()
    }
}
mod arguments;
mod assistant;
mod buffered;
mod calls;
mod streaming;
mod text;
mod tool;
mod tools;
