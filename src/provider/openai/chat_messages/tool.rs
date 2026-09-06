//! Tool-result conversion for OpenAI chat history.

use crate::provider::{ContentPart, Message};
use anyhow::Result;
use async_openai::types::chat::{
    ChatCompletionRequestMessage, ChatCompletionRequestToolMessageArgs,
};
use serde_json::{Value, json};

pub(super) fn convert(message: &Message) -> Result<Vec<ChatCompletionRequestMessage>> {
    message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => Some((tool_call_id, content)),
            _ => None,
        })
        .map(|(tool_call_id, content)| {
            Ok(ChatCompletionRequestToolMessageArgs::default()
                .tool_call_id(tool_call_id.clone())
                .content(content.clone())
                .build()?
                .into())
        })
        .collect()
}

pub(in crate::provider::openai) fn tool_json(msg: &Message) -> Vec<Value> {
    msg.content
        .iter()
        .filter_map(|part| match part {
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => Some(json!({
                "role": "tool",
                "tool_call_id": tool_call_id,
                "content": content,
            })),
            _ => None,
        })
        .collect()
}
