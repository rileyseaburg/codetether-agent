//! Ordered user text and images for chat or Responses.
use crate::*;
// Responses shape matches openai/codex e01f38c388f4907f02ac5b4980a37487686204c8,
// codex-rs/protocol/src/models.rs: ContentItem::{InputText, InputImage}.
/// Message wire format used for user image blocks.
///
/// Variants select Responses input items or chat-completion content.
///
/// # Examples
/// ```
/// use codetether_codex_protocol::UserImageFormat;
/// let format = UserImageFormat::Responses;
/// assert!(matches!(format, UserImageFormat::Responses));
/// ```
pub enum UserImageFormat {
    /// Responses API input text/image items.
    Responses,
    /// Chat-completion text/image objects.
    Chat,
}

impl CodexProtocol {
    /// Detect image parts in a message.
    ///
    /// # Arguments
    /// Accepts provider-neutral message/tool content or the mutable request
    /// projection and known call identifiers used during conversion.
    ///
    /// # Returns
    /// The encoded value or, for append methods, updates the supplied projection.
    ///
    /// # Examples
    /// See [CodexProtocol::convert_messages_to_responses_input] for the request encoding entry point.
    pub fn user_has_images(message: &Message) -> bool {
        message
            .content
            .iter()
            .any(|part| matches!(part, ContentPart::Image { .. }))
    }

    /// Encode ordered text and images for the chosen wire format.
    ///
    /// # Arguments
    /// Accepts provider-neutral message/tool content or the mutable request
    /// projection and known call identifiers used during conversion.
    ///
    /// # Returns
    /// The encoded value or, for append methods, updates the supplied projection.
    ///
    /// # Examples
    /// See [CodexProtocol::convert_messages_to_responses_input] for the request encoding entry point.
    pub fn user_image_content(message: &Message, format: UserImageFormat) -> Vec<Value> {
        message
            .content
            .iter()
            .filter_map(|part| match (&format, part) {
                (UserImageFormat::Responses, ContentPart::Text { text }) => {
                    Some(json!({ "type": "input_text", "text": text }))
                }
                (UserImageFormat::Responses, ContentPart::Image { url, .. }) => {
                    Some(json!({ "type": "input_image", "image_url": url }))
                }
                (UserImageFormat::Chat, ContentPart::Text { text }) => {
                    Some(json!({ "type": "text", "text": text }))
                }
                (UserImageFormat::Chat, ContentPart::Image { url, .. }) => {
                    Some(json!({ "type": "image_url", "image_url": { "url": url } }))
                }
                _ => None,
            })
            .collect()
    }
}
