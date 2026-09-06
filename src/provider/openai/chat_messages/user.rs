//! Typed user content shared by SDK and raw-SSE chat serialization.

use crate::provider::{ContentPart, Message};
use anyhow::Result;
use async_openai::types::chat::{
    ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPartImage as Image,
    ChatCompletionRequestMessageContentPartText as Text, ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestUserMessageContent as Content,
    ChatCompletionRequestUserMessageContentPart as Part, ImageUrl,
};

pub(super) fn convert(message: &Message) -> Result<ChatCompletionRequestMessage> {
    Ok(ChatCompletionRequestUserMessageArgs::default()
        .content(content(message))
        .build()?
        .into())
}

pub(in crate::provider::openai) fn content(message: &Message) -> Content {
    if !message
        .content
        .iter()
        .any(|p| matches!(p, ContentPart::Image { .. }))
    {
        return Content::Text(super::text::joined(message));
    }
    Content::Array(
        message
            .content
            .iter()
            .filter_map(|part| match part {
                ContentPart::Text { text } => Some(Part::Text(Text { text: text.clone() })),
                ContentPart::Image { url, .. } => Some(Part::ImageUrl(Image {
                    image_url: ImageUrl {
                        url: url.clone(),
                        detail: None,
                    },
                })),
                _ => None,
            })
            .collect(),
    )
}
