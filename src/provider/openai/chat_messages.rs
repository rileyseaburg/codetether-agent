//! Conversion of provider-neutral history into OpenAI chat messages.

use crate::provider::{Message, Role};
use anyhow::Result;
use async_openai::types::chat::{
    ChatCompletionRequestDeveloperMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs,
};

#[path = "chat_messages/assistant.rs"]
mod assistant;
#[path = "chat_messages/attachments.rs"]
pub(super) mod attachments;
#[path = "chat_messages/text.rs"]
mod text;
#[path = "chat_messages/tool.rs"]
pub(super) mod tool;
#[path = "chat_messages/user.rs"]
pub(super) mod user;

pub(super) fn convert(messages: &[Message]) -> Result<Vec<ChatCompletionRequestMessage>> {
    let mut result = Vec::new();
    for message in &attachments::transport_history(messages) {
        let content = text::joined(message);
        match message.role {
            Role::System => result.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(content)
                    .build()?
                    .into(),
            ),
            Role::Developer => result.push(
                ChatCompletionRequestDeveloperMessageArgs::default()
                    .content(content)
                    .build()?
                    .into(),
            ),
            Role::User => result.push(user::convert(message)?),
            Role::Assistant => result.push(assistant::convert(message, content)?),
            Role::Tool => result.extend(tool::convert(message)?),
        }
    }
    Ok(result)
}

#[cfg(test)]
#[path = "chat_messages/tests.rs"]
mod tests;
