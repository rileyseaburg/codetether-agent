//! Assistant message and tool-call conversion.

use crate::provider::{ContentPart, Message};
use anyhow::Result;
use async_openai::types::chat::{
    ChatCompletionMessageToolCall, ChatCompletionMessageToolCalls,
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage, FunctionCall,
};

pub(super) fn convert(message: &Message, content: String) -> Result<ChatCompletionRequestMessage> {
    let tool_calls = message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => Some(ChatCompletionMessageToolCalls::Function(
                ChatCompletionMessageToolCall {
                    id: id.clone(),
                    function: FunctionCall {
                        name: name.clone(),
                        arguments: arguments.clone(),
                    },
                },
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut builder = ChatCompletionRequestAssistantMessageArgs::default();
    if !content.is_empty() {
        builder.content(content);
    }
    if !tool_calls.is_empty() {
        builder.tool_calls(tool_calls);
    }
    Ok(builder.build()?.into())
}
