//! append responses user.
use crate::*;

impl CodexProtocol {
    /// Append user text and image input.
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
    pub fn append_responses_user(message: &Message, input: &mut Vec<Value>) {
        let content = if Self::user_has_images(message) {
            Self::user_image_content(message, UserImageFormat::Responses)
        } else {
            let text = Self::message_text(message, "\n");
            if text.is_empty() {
                return;
            }
            vec![json!({ "type": "input_text", "text": text })]
        };
        input.push(json!({
            "type": "message",
            "role": "user",
            "content": content,
        }));
    }
}
